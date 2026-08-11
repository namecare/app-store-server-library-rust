#!/usr/bin/env bash
# Runs every available language arm and renders the comparison table.
#
# A missing toolchain skips its arm with a warning rather than failing the run:
# a three-language table is useful, a hard failure is not. Skipped and failed
# arms are named on stderr and in RESULTS.md — a silently absent column reads
# as "we measured everything" when we did not.
#
# Deliberately NOT `set -e`: one arm falling over must not take the run with it.
set -uo pipefail

cd "$(dirname "$0")" || exit 1
mkdir -p results
SKIPPED=()

run_arm() {
  local name="$1" tool="$2"
  shift 2
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "warning: $tool not found - skipping the $name arm" >&2
    SKIPPED+=("$name (no $tool on PATH)")
    rm -f "results/$name.jsonl"
    return
  fi
  echo "running $name arm..." >&2
  if ! "$@" > "results/$name.jsonl" 2>"results/$name.err"; then
    echo "warning: the $name arm failed - see results/$name.err" >&2
    SKIPPED+=("$name (failed; stderr in results/$name.err)")
    rm -f "results/$name.jsonl"
    return
  fi
  rm -f "results/$name.err"
}

# The Rust arm is a plain `std::time::Instant` loop in src/bin/runner.rs, the
# same shape as the other three, and it prints the JSONL contract directly —
# so it needs no special handling and no output to parse.

run_arm rust cargo \
  cargo run -p app-store-server-library-bench-compare --bin runner --release --quiet

run_arm swift swift \
  bash -c 'cd swift && swift build -c release >&2 && exec .build/release/runner'

run_arm node node \
  bash -c 'cd node && exec node runner.mjs'

run_arm python python3 \
  bash -c 'cd python && { [ -d .venv ] || python3 -m venv .venv >&2; } && .venv/bin/pip install --quiet -r requirements.txt >&2 && exec .venv/bin/python runner.py'

printf '%s\n' "${SKIPPED[@]+"${SKIPPED[@]}"}" > results/skipped.txt

if command -v python3 >/dev/null 2>&1; then
  python3 render.py
else
  echo "warning: python3 not found - cannot render RESULTS.md" >&2
fi

exit 0
