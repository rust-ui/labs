#!/bin/bash

# Script to copy CSS animated icons from ICONS___STAGING to RUST-UI icons directory
# Path to the staging icons directory
STAGING_ICONS_PATH="./ICONS___STAGING"
RUST_UI_ICONS_PATH="../../RUST-UI/crates/icons/ICONS"

echo "CSS Animated Icons - Copy from Staging to RUST-UI:"
echo "=================================================="

echo ""
echo "From ICONS___STAGING directory:"
echo "------------------------------"
# Find all .css files and extract just the filename without extension
find "$STAGING_ICONS_PATH" -name "*.css" -type f | while read -r file; do
    # Extract filename without path and extension
    basename "$file" .css
done | sort

STAGING_COUNT=$(find "$STAGING_ICONS_PATH" -name "*.css" -type f | wc -l | tr -d ' ')
echo ""
echo "Total CSS animated icons to copy: $STAGING_COUNT"

echo ""
echo "Copying CSS files to RUST-UI icons directory..."
cp "$STAGING_ICONS_PATH"/*.css "$RUST_UI_ICONS_PATH"/

echo "Copy completed!"

echo ""
echo "Verification - CSS files now in RUST-UI directory:"
echo "-------------------------------------------------"
find "$RUST_UI_ICONS_PATH" -name "*.css" -type f | while read -r file; do
    basename "$file" .css
done | sort

FINAL_COUNT=$(find "$RUST_UI_ICONS_PATH" -name "*.css" -type f | wc -l | tr -d ' ')
echo ""
echo "Total CSS animated icons in RUST-UI: $FINAL_COUNT"

echo ""
echo "Moving all files from STAGING to DONE directory..."
# Create ICONS_DONE directory if it doesn't exist
mkdir -p ./ICONS_DONE

# Move all files from STAGING to DONE
mv "$STAGING_ICONS_PATH"/* ./ICONS_DONE/

echo "Files moved to ICONS_DONE directory!"

echo ""
echo "Process completed successfully!"
echo "- CSS files copied to RUST-UI icons directory"
echo "- All staging files moved to ICONS_DONE directory"