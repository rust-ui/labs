#!/bin/bash

# Script to list all CSS animated icons in the RUST-UI icons crate
# Path to the RUST-UI icons directory (relative path)
ICONS_PATH="../../RUST-UI/crates/icons/ICONS"

echo "CSS Animated Icons Available:"
echo "============================="

# Find all .css files and extract just the filename without extension
find "$ICONS_PATH" -name "*.css" -type f | while read -r file; do
    # Extract filename without path and extension
    basename "$file" .css
done | sort

echo ""
echo "Total CSS animated icons: $(find "$ICONS_PATH" -name "*.css" -type f | wc -l | tr -d ' ')"