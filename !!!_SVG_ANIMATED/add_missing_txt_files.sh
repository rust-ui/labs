#!/bin/bash

# Convert TSX files from kebab-case to snake_case and handle missing TXT files
cd "ICONS_WIP" || exit 1

echo "Converting TSX files from kebab-case to snake_case:"
echo "------------------------------------------------"

# First, rename all .tsx files from kebab-case to snake_case
for tsx in *.tsx; do
    if [[ "$tsx" == *"-"* ]]; then
        base="${tsx%.tsx}"
        snake_case="${base//-/_}"
        new_name="${snake_case}.tsx"
        echo "Renaming: $tsx -> $new_name"
        mv "$tsx" "$new_name"
    fi
done

echo ""
echo "Checking for TSX files missing corresponding TXT files:"
echo "-----------------------------------------------------"

missing_files=()

for tsx in *.tsx; do
    base="${tsx%.tsx}"
    txt_file="${base}.txt"
    if [[ ! -f "$txt_file" ]]; then
        echo "- $tsx -> $txt_file"
        missing_files+=("$txt_file")
    fi
done

if [[ ${#missing_files[@]} -eq 0 ]]; then
    echo "✅ All TSX files have corresponding TXT files!"
else
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
fi

echo "Done!"