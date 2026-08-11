#!/usr/bin/env python3
"""Render the cross-language comparison table from the arms' JSONL output.

Reads every ``results/*.jsonl`` plus ``results/skipped.txt``, writes
``RESULTS.md`` and prints the same table to stdout.

Two deliberate choices:

* A case an arm did not report renders as ``unsupported`` with a footnote, never
  as a blank or a zero. A blank cell reads as "slow"; the truth is usually "this
  library cannot do this at all", which is a finding, not a gap.
* The machine and date are not stamped automatically. These are wall-clock
  numbers from one machine: they rank, they do not port, so a human fills the
  line in with the box they actually ran on.
"""

from __future__ import annotations

import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RESULTS_DIR = os.path.join(HERE, "results")
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

# Column order. Rust is the baseline for the "vs Rust" ratios.
BASELINE = "rust"
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


def read_results():
    """-> ({lib: {case: ns_per_op}}, {lib: {case: iterations}})"""
    timings: dict[str, dict[str, float]] = {}
    iters: dict[str, dict[str, int]] = {}
    if not os.path.isdir(RESULTS_DIR):
        return timings, iters
    for fname in sorted(os.listdir(RESULTS_DIR)):
        if not fname.endswith(".jsonl"):
            continue
        path = os.path.join(RESULTS_DIR, fname)
        with open(path, "r", encoding="utf-8") as fh:
            for line in fh:
                line = line.strip()
                if not line:
                    continue
                try:
                    row = json.loads(line)
                except json.JSONDecodeError:
                    print(f"warning: unparseable line in {fname}: {line!r}",
                          file=sys.stderr)
                    continue
                lib = row.get("lib")
                case = row.get("case")
                ns = row.get("ns_per_op")
                if lib is None or case is None or ns is None:
                    continue
                timings.setdefault(lib, {})[case] = float(ns)
                if row.get("iterations") is not None:
                    iters.setdefault(lib, {})[case] = int(row["iterations"])
    return timings, iters


def read_skipped() -> list[str]:
    path = os.path.join(RESULTS_DIR, "skipped.txt")
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

    # No Rust arm means no baseline, so the ratio columns are dropped rather
    # than rendered as a column of dashes under a header promising a comparison
    # that was never made.
    have_baseline = BASELINE in timings

    header = ["Case"]
    align = ["---"]
    for lib in libs:
        header.append(LIB_LABEL.get(lib, lib))
        align.append("---:")
        if have_baseline and lib != BASELINE:
            header.append("vs Rust")
            align.append("---:")

    lines = ["| " + " | ".join(header) + " |",
             "| " + " | ".join(align) + " |"]

    for case in CASES:
        base = timings.get(BASELINE, {}).get(case)
        cells = [f"`{case}`"]
        for lib in libs:
            ns = timings.get(lib, {}).get(case)
            if ns is None:
                marker = footnote_ids[(lib, case)]
                cells.append(f"unsupported[^{marker}]")
                if have_baseline and lib != BASELINE:
                    cells.append("&mdash;")
                continue
            cells.append(format_ns(ns))
            if have_baseline and lib != BASELINE:
                cells.append(f"{ns / base:.1f}×" if base else "&mdash;")
        lines.append("| " + " | ".join(cells) + " |")

    return lines, libs, have_baseline


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

    table, libs, have_baseline = build_table(timings, footnote_ids)

    iteration_counts = sorted({n for per in iters.values() for n in per.values()})
    if len(iteration_counts) == 1:
        iter_note = (f"Every figure is the **median** of {iteration_counts[0]} samples "
                     f"per case (after warmup).")
    elif iteration_counts:
        iter_note = ("Every figure is the **median** of that arm's samples per case "
                     "(after warmup); sample counts: "
                     + ", ".join(str(n) for n in iteration_counts) + ".")
    else:
        iter_note = "Every figure is the **median** of that arm's samples per case."
    iter_note += (" Median rather than mean because benchmark noise only ever makes a "
                  "sample slower, so one outlier moves the mean and not the median — "
                  "see [README.md](README.md#why-median).")

    doc: list[str] = []
    doc.append("# Cross-language comparison results")
    doc.append("")
    doc.append("Generated by `bench/compare/run.sh`. Regenerate with `./run.sh`, "
               "or re-render existing results with `python3 render.py`.")
    doc.append("")
    doc.append("**Machine / date: _______________________________________** "
               "(fill in by hand — wall-clock numbers rank on one machine and do "
               "not port between machines, so an auto-stamp would be a lie the "
               "moment the file is copied.)")
    doc.append("")
    doc.append(iter_note)
    doc.append("")
    if not have_baseline:
        doc.append("> The Rust arm did not run, so there is no baseline and the "
                   "`vs Rust` ratio columns are omitted.")
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
    doc.append("## Reading this table")
    doc.append("")
    doc.append("These numbers are not a like-for-like algorithm comparison. Swift's "
               "verify methods are `async` and its `ChainVerifier` is an `actor`; "
               "Node's return promises; Rust's and Python's are plain synchronous "
               "calls. The Swift and Node figures therefore carry executor, "
               "promise-scheduling and actor-hop overhead that Rust and Python never "
               "pay. Each arm also sits on different crypto: aws-lc-rs, swift-crypto, "
               "Node's OpenSSL bindings, Python's `cryptography`. See "
               "[README.md](README.md) for the full caveats.")
    doc.append("")

    text = "\n".join(doc)
    with open(OUT_PATH, "w", encoding="utf-8") as fh:
        fh.write(text)

    print(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
