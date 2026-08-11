#!/usr/bin/env python3
"""Turn a criterion run into a markdown delta table.

Reads `target/criterion/` rather than `cargo bench` stdout. Criterion's
terminal output is formatted for humans and its wording has changed between
versions; the JSON under each benchmark directory has not, and it carries the
confidence intervals that decide whether a delta means anything.

Three files per benchmark matter here:

  new/benchmark.json     the real benchmark id, e.g. `verifier/trivial_chain_building`
                         (the directory name is flattened and lossy)
  new/estimates.json     this run's absolute time, in nanoseconds
  change/estimates.json  the delta against the baseline, as a ratio, with a
                         confidence interval — absent when there is nothing
                         to compare against

A benchmark directory is any directory containing `new/estimates.json`. They
are found by walking rather than listing, because criterion nests a level
deeper for grouped or parameterized benchmarks.

Stale directories are excluded by modification time, via `--since`. When
`target/criterion/` carries state from before the run — a restored baseline
cache, or a workspace that happened to persist — a benchmark that was
renamed, removed, or simply not selected by this run still has a directory
sitting there holding old numbers. Reporting those as current results is the
failure this guards against.

The output is a table on stdout and a compact JSON summary on stderr for the
workflow to act on.
"""

from __future__ import annotations

import argparse
import json
import os
import sys
from dataclasses import dataclass
from pathlib import Path

# Flag a benchmark only when the change is both large enough to care about and
# statistically distinguishable from noise. Either test alone misreports: a
# fixed percentage flags jitter in the nanosecond-scale policy benchmarks,
# while significance alone flags a rock-steady 0.5% drift that no one can act
# on.
THRESHOLD = float(os.environ.get("REGRESSION_THRESHOLD", "0.10"))


@dataclass
class Bench:
    name: str
    new_ns: float
    change: float | None  # ratio, e.g. 0.05 for +5%
    significant: bool

    @property
    def status(self) -> str:
        if self.change is None:
            return "new"
        if not self.significant or abs(self.change) < THRESHOLD:
            return "same"
        return "regressed" if self.change > 0 else "improved"


def humanize(ns: float) -> str:
    """Criterion reports nanoseconds; show whichever unit keeps 3-4 digits."""
    for limit, unit, scale in (
        (1_000, "ns", 1),
        (1_000_000, "µs", 1_000),
        (1_000_000_000, "ms", 1_000_000),
    ):
        if ns < limit:
            return f"{ns / scale:.3g} {unit}"
    return f"{ns / 1_000_000_000:.3g} s"


def read(bench_dir: Path) -> Bench | None:
    meta_path = bench_dir / "new" / "benchmark.json"
    new_path = bench_dir / "new" / "estimates.json"
    if not meta_path.is_file() or not new_path.is_file():
        return None

    meta = json.loads(meta_path.read_text())
    new = json.loads(new_path.read_text())

    # Prefer the slope estimate: for a linear-sampled benchmark it uses every
    # sample, so it is steadier than the mean. Criterion omits it for flat
    # sampling, where the mean is what it reports itself.
    estimate = new.get("slope") or new["mean"]
    new_ns = estimate["point_estimate"]

    change_path = bench_dir / "change" / "estimates.json"
    if not change_path.is_file():
        return Bench(meta["full_id"], new_ns, None, False)

    change = json.loads(change_path.read_text())["mean"]
    ratio = change["point_estimate"]
    ci = change["confidence_interval"]
    # A confidence interval straddling zero means the change is indistinguishable
    # from noise, which is exactly criterion's own significance criterion.
    significant = not (ci["lower_bound"] <= 0.0 <= ci["upper_bound"])

    return Bench(meta["full_id"], new_ns, ratio, significant)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "root",
        nargs="?",
        default="target/criterion",
        type=Path,
        help="criterion output directory (default: target/criterion)",
    )
    parser.add_argument(
        "--since",
        type=float,
        default=None,
        metavar="EPOCH",
        help="ignore benchmarks not rewritten since this unix timestamp, "
        "dropping results left behind by earlier runs",
    )
    parser.add_argument(
        "--base",
        default="master",
        metavar="BRANCH",
        help="branch the baseline was recorded on, named in the report",
    )
    args = parser.parse_args()

    root: Path = args.root
    if not root.is_dir():
        print(f"No criterion output at `{root}`.", file=sys.stderr)
        return 1

    # `report/` mirrors the benchmark tree with rendered HTML and would yield
    # phantom entries, so it is skipped wherever it appears.
    candidates = [
        d
        for d in root.rglob("*")
        if d.is_dir() and "report" not in d.relative_to(root).parts
    ]

    if args.since is not None:
        candidates = [
            d
            for d in candidates
            if (e := d / "new" / "estimates.json").is_file()
            and e.stat().st_mtime >= args.since
        ]

    benches = sorted(
        (b for d in candidates if (b := read(d))),
        key=lambda b: b.name,
    )
    if not benches:
        print(f"No benchmarks found under `{root}`.", file=sys.stderr)
        return 1

    regressed = [b for b in benches if b.status == "regressed"]
    improved = [b for b in benches if b.status == "improved"]
    added = [b for b in benches if b.status == "new"]

    icon = {"regressed": "🔴", "improved": "🟢", "same": "▫️", "new": "🆕"}

    # A stable, invisible marker so CI can find its own previous comment and
    # edit it in place instead of stacking a new one on every push.
    out: list[str] = ["<!-- criterion-benchmark-report -->"]
    if added and len(added) == len(benches):
        # Every benchmark lacking a delta means there was no baseline at all
        # — a state that must not masquerade as "no significant change".
        out.append(
            f"⚠️ **No `{args.base}` baseline found** — nothing was compared. "
            f"The next benchmarked push to `{args.base}` records one."
        )
    elif regressed:
        noun = "benchmark" if len(regressed) == 1 else "benchmarks"
        out.append(f"🔴 **{len(regressed)} {noun} regressed** past {THRESHOLD:.0%}.")
    elif improved:
        noun = "benchmark" if len(improved) == 1 else "benchmarks"
        out.append(f"🟢 No regressions — {len(improved)} {noun} improved.")
    else:
        out.append("▫️ No significant change.")
    out.append("")

    out.append("| | Benchmark | Base | This PR | Change |")
    out.append("|---|---|---|---|---|")
    for b in benches:
        if b.change is None:
            out.append(f"| {icon[b.status]} | `{b.name}` | — | {humanize(b.new_ns)} | new |")
            continue
        # `new_ns` is the measured time and `change` the ratio against the
        # baseline, so the baseline is recovered rather than re-read.
        base_ns = b.new_ns / (1.0 + b.change)
        delta = f"{b.change:+.1%}"
        if b.status == "same":
            delta += " *(noise)*" if abs(b.change) >= THRESHOLD else ""
        out.append(
            f"| {icon[b.status]} | `{b.name}` | {humanize(base_ns)} "
            f"| {humanize(b.new_ns)} | {delta} |"
        )

    out.append("")
    details = [f"{len(benches)} benchmarks"]
    if added:
        details.append(f"{len(added)} new")
    compared = (
        f"Compared against the latest <code>{args.base}</code> baseline."
        if len(added) < len(benches)
        else "No baseline was available to compare against."
    )
    out.append(
        f"<sub>{', '.join(details)}. Flagged at ≥{THRESHOLD:.0%} change with a "
        "confidence interval excluding zero; anything else is reported as noise. "
        f"{compared}</sub>"
    )

    print("\n".join(out))
    json.dump(
        {
            "total": len(benches),
            "regressed": [b.name for b in regressed],
            "improved": [b.name for b in improved],
            "new": [b.name for b in added],
        },
        sys.stderr,
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
