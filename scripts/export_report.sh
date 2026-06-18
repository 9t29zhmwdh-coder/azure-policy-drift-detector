#!/usr/bin/env bash
set -e

OUTPUT_DIR="${1:-./reports}"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)

mkdir -p "$OUTPUT_DIR"

./target/release/apdd export --format json --output "$OUTPUT_DIR/report-$TIMESTAMP.json"
./target/release/apdd export --format md --output "$OUTPUT_DIR/report-$TIMESTAMP.md"

echo "Reports written to $OUTPUT_DIR/"
