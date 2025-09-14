#!/bin/bash

# Check for TSX files that don't have corresponding TXT files in ICONS_WIP
cd "ICONS_WIP" || exit 1
echo "TSX files missing corresponding TXT files:"
echo "----------------------------------------"

missing_files=()

for tsx in *.tsx; do
    base="${tsx%.tsx}"
    snake_case="${base//-/_}"
    if [[ ! -f "${snake_case}.txt" ]]; then
        echo "- $tsx (missing ${snake_case}.txt)"
        missing_files+=("${snake_case}.txt")
    fi
done

echo ""
echo "Moving missing files from ../ICONS_TODOS to ./ICONS_WIP:"
echo "--------------------------------------------------------"

for file in "${missing_files[@]}"; do
    if [[ -f "../ICONS_TODOS/$file" ]]; then
        echo "Moving $file..."
        mv "../ICONS_TODOS/$file" "./$file"
    else
        echo "⚠️  Warning: $file not found in ../ICONS_TODOS"
    fi
done

echo "Done!"