#!/usr/bin/env bash

set -uo pipefail

cd "$(dirname "$0")" || exit 1

OUTPUT_DIR="${OUTPUT_DIR:-.output}"
BACKENDS=(aws_lc rust_crypto ring)
BENCHES=(verify sign)
SAMPLE_COUNT=500
SAMPLE_SIZE=1

mkdir -p "$OUTPUT_DIR"
SKIPPED=()

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo not found" >&2
  exit 1
fi

for backend in "${BACKENDS[@]}"; do
  for bench in "${BENCHES[@]}"; do
    raw="$OUTPUT_DIR/$backend.$bench.txt"
    err="$OUTPUT_DIR/$backend.$bench.err"
    echo "running $backend/$bench..." >&2
    if ! COMPARE_BACKEND="$backend" cargo bench -q \
           --no-default-features --features "$backend" --bench "$bench" -- \
           --color never --sample-count "$SAMPLE_COUNT" --sample-size "$SAMPLE_SIZE" \
         > "$raw" 2>"$err"; then
      echo "warning: the $backend/$bench arm failed - see $err" >&2
      SKIPPED+=("$backend/$bench (failed; stderr in $err)")
      rm -f "$raw"
    fi
  done
done

printf '%s\n' "${SKIPPED[@]+"${SKIPPED[@]}"}" > "$OUTPUT_DIR/skipped.txt"

if command -v python3 >/dev/null 2>&1; then
  OUTPUT_DIR="$OUTPUT_DIR" python3 render.py
else
  echo "warning: python3 not found - cannot render RESULTS.md" >&2
fi

exit 0
