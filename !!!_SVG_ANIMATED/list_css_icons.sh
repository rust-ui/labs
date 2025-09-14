#!/bin/bash

# Script to list all CSS animated icons in the ICONS___STAGING directory
# Path to the staging icons directory
STAGING_ICONS_PATH="./ICONS___STAGING"

echo "CSS Animated Icons Available:"
echo "============================="

echo ""
echo "From ICONS___STAGING directory:"
echo "------------------------------"
# Find all .css files and extract just the filename without extension
find "$STAGING_ICONS_PATH" -name "*.css" -type f | while read -r file; do
    # Extract filename without path and extension
    basename "$file" .css
done | sort

echo ""
echo "Total CSS animated icons: $(find "$STAGING_ICONS_PATH" -name "*.css" -type f | wc -l | tr -d ' ')"