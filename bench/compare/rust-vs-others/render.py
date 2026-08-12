#!/usr/bin/env python3

from __future__ import annotations

import json
import os
import re
import statistics
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RESULTS_DIR = os.path.join(HERE, "results")
RAW_DIR = os.path.join(RESULTS_DIR, "raw")
OUT_PATH = os.path.join(HERE, "RESULTS.md")

# Row order for the table.
CASES = [
    "verify_notification",
    "verify_transaction",
    "verify_renewal_info",
    "receipt_app",
    "receipt_app_legacy",
    "sign_promotional_offer",
]

# Column order. Arms not listed here are appended alphabetically.
LIB_ORDER = ["rust", "swift", "node", "python"]
LIB_LABEL = {
    "rust": "Rust",
    "swift": "Swift",
    "node": "Node",
    "python": "Python",
}

# Why a given arm cannot do a given case. Keyed (lib, case); anything not listed
# gets the generic footnote.
KNOWN_UNSUPPORTED = {
    ("node", "receipt_app"): (
        "Node's `@apple/app-store-server-library` has a DER-only ASN.1 parser and "
        "rejects the BER indefinite-length app receipt (header `30 80`) with "
        "\"too short ASN.1 value\". Rust, Swift and Python all parse the same fixture."
    ),
}


class PreReduced(float):
    """A median a harness already computed, where raw samples are unusable.
    """


def _median(values: list[float]) -> float:
    return statistics.median(values)


def parse_rust(path: str) -> dict[str, float]:
    """Divan's console table.

    Looks like, with UTF-8 box drawing and no ANSI escapes under `--color never`:

        compare                    fastest   │ slowest   │ median    │ mean      │ samples │ iters
        ├─ receipt_app             2.665 µs  │ 64.58 µs  │ 2.791 µs  │ 2.933 µs  │ 500     │ 500
        ╰─ verify_transaction      132.7 µs  │ 195.8 µs  │ 142.9 µs  │ 142.5 µs  │ 500     │ 500

    Note there is NO separator between the name and `fastest`: they share the
    first field, so a split on `│` yields six fields and `median` is at index 2.

    Rows are keyed by name rather than position: Divan sorts its tree
    alphabetically, not in the canonical case order.
    """
    out: dict[str, float] = {}
    with open(path, "r", encoding="utf-8") as fh:
        for line in fh:
            if "│" not in line:
                continue
            columns = [c.strip() for c in line.split("│")]
            # (name + fastest), slowest, median, mean, samples, iters
            if len(columns) < 6:
                continue
            # The name runs up to the first run of 2+ spaces before `fastest`.
            head = columns[0].lstrip("├╰│─ ").strip()
            name = re.split(r"\s{2,}", head)[0].strip()
            if name not in CASES:
                continue
            ns = _parse_duration(columns[2])
            if ns is not None:
                out[name] = PreReduced(ns)
    return out


def _parse_duration(cell: str) -> float | None:
    """"142.9 µs" -> 142900.0. Divan auto-scales the unit per row."""
    parts = cell.split()
    if len(parts) != 2:
        return None
    try:
        value = float(parts[0])
    except ValueError:
        return None
    scale = {"ns": 1.0, "µs": 1e3, "us": 1e3, "ms": 1e6, "s": 1e9}.get(parts[1])
    return value * scale if scale else None


def parse_swift(directory: str) -> dict[str, float]:
    """`package-benchmark`'s `--format histogram` export.

    One file per case, named
    ``Current_run.Bench.<case>.wallClock.histogram.txt``, holding an
    HDR histogram as ``value percentile totalCount 1/(1-percentile)`` rows:

        Value     Percentile TotalCount 1/(1-Percentile)
        19167.000 0.000000000000          1           1.00
        24255.000 0.500000000000        252           2.00

    Values are in NANOSECONDS at full precision, which is why this format is
    used rather than `histogramSamples`. That one exports the raw per-iteration
    samples, but rounds them to whole display units — verified: every sample in
    a `histogramSamples` export of these same cases was a whole number of
    microseconds, with a minimum gap between distinct values of exactly 1.000 µs.
    For an ~24 µs case that is 4% granularity, the same limitation that made
    XCTest `measure {}` unusable here.

    So Swift is the one arm besides Rust whose median comes pre-reduced: the
    p50 row of the histogram. It is a true nanosecond-resolution p50 over all
    500 samples, just not something this module computes itself.
    """
    out: dict[str, float] = {}
    if not os.path.isdir(directory):
        return out
    for fname in sorted(os.listdir(directory)):
        if not fname.endswith(".histogram.txt"):
            continue
        parts = fname.split(".")
        # Current_run . <target> . <case> . wallClock . histogram . txt
        if len(parts) < 3:
            continue
        name = parts[2]
        if name not in CASES:
            continue
        p50 = None
        with open(os.path.join(directory, fname), "r", encoding="utf-8") as fh:
            for line in fh:
                fields = line.split()
                if len(fields) < 2:
                    continue
                try:
                    value, percentile = float(fields[0]), float(fields[1])
                except ValueError:
                    continue  # header and the trailing #[Mean = ...] lines
                # The first row at or past the median. HDR histograms emit many
                # rows, so this takes the earliest one rather than the last.
                if percentile >= 0.5:
                    p50 = value
                    break
        if p50 is not None:
            out[name] = PreReduced(p50)
    return out


def parse_node(path: str) -> dict[str, list[float]]:
    """tinybench results, written as JSON by node/runner.mjs already in ns."""
    out: dict[str, list[float]] = {}
    with open(path, "r", encoding="utf-8") as fh:
        payload = json.load(fh)
    for name, entry in payload.get("cases", {}).items():
        if name not in CASES:
            continue
        samples = entry.get("samples_ns") or []
        if samples:
            out[name] = [float(s) for s in samples]
        elif entry.get("p50_ns") is not None:
            out[name] = PreReduced(float(entry["p50_ns"]))
    return out


def parse_python(path: str) -> dict[str, list[float]]:
    """pytest-benchmark's --benchmark-json report.

    Case names come back as `test_case[verify_notification]`; `stats.data` holds
    the raw per-round samples, in seconds.
    """
    out: dict[str, list[float]] = {}
    with open(path, "r", encoding="utf-8") as fh:
        payload = json.load(fh)
    for entry in payload.get("benchmarks", []):
        name = entry.get("param") or entry.get("name", "")
        if name.startswith("test_case["):
            name = name[len("test_case["):].rstrip("]")
        if name not in CASES:
            continue
        data = entry.get("stats", {}).get("data") or []
        if data:
            out[name] = [float(s) * 1e9 for s in data]
        elif entry.get("stats", {}).get("median") is not None:
            out[name] = PreReduced(float(entry["stats"]["median"]) * 1e9)
    return out


# Each arm's raw artifact and the parser that understands it. Swift's is a
# DIRECTORY — package-benchmark's histogram export writes one file per case —
# while the other three are single files.
PARSERS = {
    "rust": ("rust.txt", parse_rust),
    "swift": ("swift", parse_swift),
    "node": ("node.json", parse_node),
    "python": ("python.json", parse_python),
}


def read_results():
    """-> ({lib: {case: ns_per_op}}, {lib: {case: sample_count}})
    """
    timings: dict[str, dict[str, float]] = {}
    counts: dict[str, dict[str, int]] = {}
    if not os.path.isdir(RAW_DIR):
        print(f"warning: no {RAW_DIR}; run ./run.sh first", file=sys.stderr)
        return timings, counts

    for lib, (fname, parser) in PARSERS.items():
        path = os.path.join(RAW_DIR, fname)
        if not os.path.exists(path):
            continue
        try:
            parsed = parser(path)
        except Exception as e:  # noqa: BLE001 — a bad artifact must not kill the render
            print(f"warning: could not parse {fname}: {e}", file=sys.stderr)
            continue
        for case, value in parsed.items():
            if isinstance(value, PreReduced):
                timings.setdefault(lib, {})[case] = float(value)
            else:
                timings.setdefault(lib, {})[case] = _median(value)
                counts.setdefault(lib, {})[case] = len(value)
    return timings, counts


def read_skipped() -> list[str]:
    path = os.path.join(RAW_DIR, "skipped.txt")
    if not os.path.exists(path):
        return []
    with open(path, "r", encoding="utf-8") as fh:
        return [ln.strip() for ln in fh if ln.strip()]


def format_ns(ns: float) -> str:
    """Pick a unit per value, so a 2.68us row and a 1.48ms row both read cleanly."""
    if ns < 1_000:
        return f"{ns:.1f} ns"
    if ns < 1_000_000:
        return f"{ns / 1_000:.2f} µs"
    return f"{ns / 1_000_000:.2f} ms"


def build_table(timings, footnote_ids):
    libs = [lib for lib in LIB_ORDER if lib in timings]
    libs += [lib for lib in sorted(timings) if lib not in LIB_ORDER]

    header = ["Case"] + [LIB_LABEL.get(lib, lib) for lib in libs]
    align = ["---"] + ["---:"] * len(libs)

    lines = ["| " + " | ".join(header) + " |",
             "| " + " | ".join(align) + " |"]

    for case in CASES:
        cells = [f"`{case}`"]
        for lib in libs:
            ns = timings.get(lib, {}).get(case)
            if ns is None:
                cells.append(f"unsupported[^{footnote_ids[(lib, case)]}]")
            else:
                cells.append(format_ns(ns))
        lines.append("| " + " | ".join(cells) + " |")

    return lines, libs


def main() -> int:
    timings, iters = read_results()
    skipped = read_skipped()

    # Assign footnote numbers to every missing (arm, case) pair, but only for
    # arms that actually reported something - an arm that never ran is a
    # Coverage entry, not six "unsupported" cells.
    footnote_ids: dict[tuple[str, str], int] = {}
    footnotes: list[tuple[int, str]] = []
    next_id = 1
    for case in CASES:
        for lib in LIB_ORDER:
            if lib not in timings:
                continue
            if case in timings[lib]:
                continue
            reason = KNOWN_UNSUPPORTED.get(
                (lib, case),
                f"The {LIB_LABEL.get(lib, lib)} library reported no result for "
                f"`{case}`: it exposes no equivalent API, or the call failed on "
                f"this fixture. This is an absence of capability, not a slow "
                f"measurement.",
            )
            footnote_ids[(lib, case)] = next_id
            footnotes.append((next_id, reason))
            next_id += 1

    table, libs = build_table(timings, footnote_ids)

    doc: list[str] = []
    doc.append("# Cross-language comparison results")
    doc.append("")
    doc.append("Generated by `bench/compare/rust-vs-others/run.sh`. Regenerate with `./run.sh`, "
               "or re-render existing results with `python3 render.py`.")
    doc.append("")
    doc.extend(table)
    doc.append("")

    if footnotes:
        for fid, reason in footnotes:
            doc.append(f"[^{fid}]: {reason}")
        doc.append("")

    doc.append("## Coverage")
    doc.append("")
    if skipped:
        doc.append("These arms did not contribute to the table:")
        doc.append("")
        for entry in skipped:
            doc.append(f"- {entry}")
    else:
        doc.append("Skipped or failed arms: none. Every arm ran.")
    doc.append("")
    ran = ", ".join(LIB_LABEL.get(lib, lib) for lib in libs) if libs else "none"
    doc.append(f"Arms that reported results: {ran}.")
    doc.append("")
    doc.append("")

    text = "\n".join(doc)
    with open(OUT_PATH, "w", encoding="utf-8") as fh:
        fh.write(text)

    print(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
