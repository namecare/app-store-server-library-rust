#!/usr/bin/env bash

set -uo pipefail

cd "$(dirname "$0")" || exit 1
mkdir -p results/raw
SKIPPED=()

export COMPARE_WARMUP=50
export COMPARE_ITERATIONS=500

# run_arm <name> <required-tool> <raw-artifact-extension> <command...>
#
# stdout is the raw artifact; stderr is kept next to it so a failed arm can be
# diagnosed without re-running.
run_arm() {
  local name="$1" tool="$2" ext="$3"
  shift 3
  local raw="results/raw/$name.$ext"
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "warning: $tool not found - skipping the $name arm" >&2
    SKIPPED+=("$name (no $tool on PATH)")
    rm -f "$raw"
    return
  fi
  echo "running $name arm..." >&2
  if ! "$@" > "$raw" 2>"results/raw/$name.err"; then
    echo "warning: the $name arm failed - see results/raw/$name.err" >&2
    SKIPPED+=("$name (failed; stderr in results/raw/$name.err)")
    rm -f "$raw"
    return
  fi
}

# Rust — Divan.
run_arm rust cargo txt \
  bash -c 'cd rust && exec cargo bench -q --bench compare -- \
    --color never --sample-count "$COMPARE_ITERATIONS" --sample-size 1'

# Swift — `package-benchmark`
if ! command -v swift >/dev/null 2>&1; then
  echo "warning: swift not found - skipping the swift arm" >&2
  SKIPPED+=("swift (no swift on PATH)")
elif ! { [ -f /opt/homebrew/include/jemalloc/jemalloc.h ] || \
         [ -f /usr/local/include/jemalloc/jemalloc.h ] || \
         [ -f /usr/include/jemalloc/jemalloc.h ]; }; then
  echo "warning: jemalloc not found - skipping the swift arm." >&2
  echo "         package-benchmark requires it at build time; install with:" >&2
  echo "           brew install jemalloc        (macOS)" >&2
  echo "           apt-get install libjemalloc-dev   (Debian/Ubuntu)" >&2
  SKIPPED+=("swift (jemalloc not installed; see run.sh)")
else
  echo "running swift arm..." >&2
  mkdir -p results/raw/swift
  if ! ( cd swift && exec swift package --disable-sandbox benchmark \
           --format histogram --path ../results/raw/swift --no-progress ) \
         > results/raw/swift.txt 2>results/raw/swift.err; then
    echo "warning: the swift arm failed - see results/raw/swift.err" >&2
    SKIPPED+=("swift (failed; stderr in results/raw/swift.err)")
    rm -rf results/raw/swift results/raw/swift.txt
  fi
fi

# Node — tinybench, emitting its own results as JSON including raw samples.
run_arm node node json \
  bash -c 'cd node && { [ -d node_modules ] || npm install --silent >&2; } && exec node runner.mjs'

# Python — pytest-benchmark, via --benchmark-json. The report goes to a file
# rather than stdout.
run_arm python python3 txt \
  bash -c 'cd python && { [ -d .venv ] || python3 -m venv .venv >&2; } && \
    .venv/bin/pip install --quiet -r requirements.txt >&2 && \
    exec .venv/bin/python -m pytest test_compare.py -q \
      --benchmark-json=../results/raw/python.json'

printf '%s\n' "${SKIPPED[@]+"${SKIPPED[@]}"}" > results/raw/skipped.txt

# Stage 4, a separate step: parses all four raw formats and renders the table.
# Re-runnable on its own over saved output — `python3 render.py`.
if command -v python3 >/dev/null 2>&1; then
  python3 render.py
else
  echo "warning: python3 not found - cannot render RESULTS.md" >&2
fi

exit 0
