#!/usr/bin/env bash

set -uo pipefail

cd "$(dirname "$0")" || exit 1
mkdir -p results/raw
SKIPPED=()

BACKENDS=(aws_lc rust_crypto ring)
SAMPLE_COUNT=500
SAMPLE_SIZE=1

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo not found" >&2
  exit 1
fi

for backend in "${BACKENDS[@]}"; do
  raw="results/raw/$backend.txt"
  echo "running $backend..." >&2
  if ! ( cd rust && COMPARE_BACKEND="$backend" exec cargo bench -q \
           --no-default-features --features "$backend" --bench backend -- \
           --color never --sample-count "$SAMPLE_COUNT" --sample-size "$SAMPLE_SIZE" ) \
       > "$raw" 2>"results/raw/$backend.err"; then
    echo "warning: the $backend arm failed - see results/raw/$backend.err" >&2
    SKIPPED+=("$backend (failed; stderr in results/raw/$backend.err)")
    rm -f "$raw"
  fi
done

printf '%s\n' "${SKIPPED[@]+"${SKIPPED[@]}"}" > results/raw/skipped.txt

if command -v python3 >/dev/null 2>&1; then
  python3 render.py
else
  echo "warning: python3 not found - cannot render RESULTS.md" >&2
fi

exit 0
