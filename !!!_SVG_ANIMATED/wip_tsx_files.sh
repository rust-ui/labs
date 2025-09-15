#!/bin/bash

# Script to move the first N .tsx files from icons-animated-pqoqubbw to ICONS_WIP
# With progress output and validation
# Usage: ./wip_tsx_files.sh <number_of_files>

set -e

# Check if parameter is provided
if [ $# -eq 0 ]; then
    echo "❌ Error: Number of files parameter is required"
    echo "Usage: $0 <number_of_files>"
    echo "Example: $0 20"
    exit 1
fi

# Validate parameter is a number
if ! [[ "$1" =~ ^[0-9]+$ ]]; then
    echo "❌ Error: Parameter must be a positive number"
    echo "Usage: $0 <number_of_files>"
    exit 1
fi

SOURCE_DIR="./icons-animated-pqoqubbw"
TARGET_DIR="./ICONS_WIP"
MAX_FILES=$1

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔄 Moving first ${MAX_FILES} .tsx files from ${SOURCE_DIR} to ${TARGET_DIR}${NC}"
echo "=================================================="

# Check if source directory exists
if [ ! -d "$SOURCE_DIR" ]; then
    echo -e "${RED}❌ Error: Source directory '$SOURCE_DIR' does not exist${NC}"
    exit 1
fi

# Create target directory if it doesn't exist
if [ ! -d "$TARGET_DIR" ]; then
    echo -e "${YELLOW}📁 Creating target directory: $TARGET_DIR${NC}"
    mkdir -p "$TARGET_DIR"
fi

# Get list of .tsx files (first 20)
echo -e "${BLUE}🔍 Finding .tsx files...${NC}"
tsx_files=($(ls -1 "$SOURCE_DIR" | grep "\.tsx$" | head -$MAX_FILES))

if [ ${#tsx_files[@]} -eq 0 ]; then
    echo -e "${RED}❌ No .tsx files found in $SOURCE_DIR${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Found ${#tsx_files[@]} .tsx files to move${NC}"
echo ""

# Move files with progress
moved_count=0
for file in "${tsx_files[@]}"; do
    source_path="$SOURCE_DIR/$file"
    target_path="$TARGET_DIR/$file"

    if [ -f "$source_path" ]; then
        if [ -f "$target_path" ]; then
            echo -e "${YELLOW}⚠️  File already exists in target: $file (skipping)${NC}"
        else
            mv "$source_path" "$target_path"
            moved_count=$((moved_count + 1))
            echo -e "${GREEN}✅ Moved: $file${NC}"
        fi
    else
        echo -e "${RED}❌ Source file not found: $file${NC}"
    fi
done

echo ""
echo "=================================================="
echo -e "${GREEN}🎉 Operation completed!${NC}"
echo -e "${GREEN}📊 Files moved: $moved_count${NC}"
echo -e "${BLUE}📂 Target directory: $TARGET_DIR${NC}"

# Show what's now in the target directory
echo ""
echo -e "${BLUE}📋 Files now in $TARGET_DIR:${NC}"
ls -la "$TARGET_DIR" | grep "\.tsx$" || echo "No .tsx files in target directory"