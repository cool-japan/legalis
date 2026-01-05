#!/bin/bash
# A simple script to publish a single crate for Legalis-RS

if [ -z "$1" ]; then
    echo "Usage: $0 <crate-directory>"
    exit 1
fi

CRATE=$1
echo "===== Publishing $CRATE ====="

cd "$CRATE" || { echo "Directory not found: $CRATE"; exit 1; }
cargo publish --allow-dirty || { echo "Failed to publish: $CRATE"; exit 1; }
echo "✓ Successfully published $CRATE"
