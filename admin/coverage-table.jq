# Renders `cargo llvm-cov --json` output as a markdown table, for the PR
# comment and job summary in .github/workflows/build_test.yml.
#
#   ROOT=$PWD jq -rf admin/coverage-table.jq coverage.json
#
# ROOT is stripped from the front of each filename; llvm-cov reports absolute
# paths, which are unreadable in a comment and specific to the runner.

def bar(p): (p / 10 | floor) as $n
  | ("█" * $n) + ("░" * (10 - $n));

def pct(p): ((p * 10 | round) / 10 | tostring) + "%";

.data[0] as $d
| ($d.totals.lines) as $t
| "| File | Lines | Covered | Coverage |",
  "|---|---:|---:|---|",
  ( $d.files
    # A file with no executable lines carries no signal and would render as a
    # misleading 0%.
    | map(select(.summary.lines.count > 0))
    # Worst first: the top of the table is the part worth acting on.
    | sort_by(.summary.lines.percent)
    | .[]
    | "| `\(.filename | sub("^" + $ENV.ROOT + "/"; ""))` "
      + "| \(.summary.lines.count) "
      + "| \(.summary.lines.covered) "
      + "| \(bar(.summary.lines.percent)) \(pct(.summary.lines.percent)) |"
  ),
  "| **TOTAL** | **\($t.count)** | **\($t.covered)** | **\(bar($t.percent)) \(pct($t.percent))** |"