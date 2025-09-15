#!/bin/bash

# Move all files from ICONS_WIP to ICONS___STAGING
# Usage: ./move_to_staging.sh

# Check if ICONS_WIP directory exists
if [ ! -d "./ICONS_WIP" ]; then
    echo "Error: ICONS_WIP directory not found"
    exit 1
fi

# Create ICONS___STAGING directory if it doesn't exist
if [ ! -d "./ICONS___STAGING" ]; then
    echo "Creating ICONS___STAGING directory..."
    mkdir -p "./ICONS___STAGING"
fi

# Check if ICONS_WIP has any files
if [ -z "$(ls -A ./ICONS_WIP)" ]; then
    echo "ICONS_WIP directory is empty - nothing to move"
    exit 0
fi

# Move all files from ICONS_WIP to ICONS___STAGING
echo "Moving files from ICONS_WIP to ICONS___STAGING..."

# Move files one by one with individual printing
for file in ./ICONS_WIP/*; do
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        echo "  Moving: $filename"
        mv "$file" "./ICONS___STAGING/"
        if [ $? -eq 0 ]; then
            echo "    ✅ $filename moved successfully"
        else
            echo "    ❌ Failed to move $filename"
        fi
    fi
done

echo ""
echo "✅ Move operation completed"
echo "Files in ICONS___STAGING:"
ls -la ./ICONS___STAGING/