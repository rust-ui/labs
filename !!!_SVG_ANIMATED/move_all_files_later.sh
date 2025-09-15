#!/bin/bash

# Create destination directory if it doesn't exist
mkdir -p ICONS__LATER

# Move all files from ICONS_WIP to ICONS__LATER
for file in ICONS_WIP/*; do
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        echo "Moving: $filename"
        mv "$file" ICONS__LATER/
    fi
done

echo "All files moved from ICONS_WIP to ICONS__LATER"