#!/bin/bash

# Check for TSX files that don't have corresponding TXT files in ICONS_WIP
cd "ICONS_WIP" || exit 1
echo "TSX files missing corresponding TXT files:"
echo "----------------------------------------"

for tsx in *.tsx; do
    base="${tsx%.tsx}"
    snake_case="${base//-/_}"
    if [[ ! -f "${snake_case}.txt" ]]; then
        echo "- $tsx (missing ${snake_case}.txt)"
    fi
done