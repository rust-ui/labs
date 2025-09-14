#!/usr/bin/env bash

# Icon Comparison Script
# Compares files between ICONS_TODOS (snake_case.txt) and icons-animated-pqoqubbw (kebab-case.tsx)
# and generates a markdown report of common files

set -e

# Directory paths (relative to script location)
ICONS_TODOS_DIR="./ICONS_TODOS"
ICONS_ANIMATED_DIR="./icons-animated-pqoqubbw"
OUTPUT_FILE="./icon_comparison_report.md"

# Temporary files
TODOS_NORMALIZED="/tmp/todos_normalized.txt"
ANIMATED_NORMALIZED="/tmp/animated_normalized.txt"
COMMON_FILES="/tmp/common_files.txt"
TODOS_ONLY="/tmp/todos_only.txt"
ANIMATED_ONLY="/tmp/animated_only.txt"

echo "🔍 Starting icon file comparison..."

# Function to normalize filename (snake_case to kebab-case, remove extension)
normalize_filename() {
    local filename="$1"
    # Remove extension
    basename_no_ext="${filename%.*}"
    # Convert snake_case to kebab-case
    echo "$basename_no_ext" | sed 's/_/-/g'
}

# Create normalized lists
echo "📋 Normalizing ICONS_TODOS filenames..."
> "$TODOS_NORMALIZED"
if [ -d "$ICONS_TODOS_DIR" ]; then
    for file in "$ICONS_TODOS_DIR"/*.txt; do
        if [ -f "$file" ]; then
            filename=$(basename "$file")
            normalized=$(normalize_filename "$filename")
            echo "$normalized" >> "$TODOS_NORMALIZED"
        fi
    done
    sort "$TODOS_NORMALIZED" -o "$TODOS_NORMALIZED"
else
    echo "❌ Error: ICONS_TODOS directory not found: $ICONS_TODOS_DIR"
    exit 1
fi

echo "📋 Normalizing icons-animated-pqoqubbw filenames..."
> "$ANIMATED_NORMALIZED"
if [ -d "$ICONS_ANIMATED_DIR" ]; then
    for file in "$ICONS_ANIMATED_DIR"/*.tsx; do
        if [ -f "$file" ]; then
            filename=$(basename "$file")
            normalized=$(normalize_filename "$filename")
            echo "$normalized" >> "$ANIMATED_NORMALIZED"
        fi
    done
    sort "$ANIMATED_NORMALIZED" -o "$ANIMATED_NORMALIZED"
else
    echo "❌ Error: icons-animated-pqoqubbw directory not found: $ICONS_ANIMATED_DIR"
    exit 1
fi

# Find common files
echo "🔗 Finding common files..."
comm -12 "$TODOS_NORMALIZED" "$ANIMATED_NORMALIZED" > "$COMMON_FILES"

# Create lookup files for better performance
echo "🔧 Creating filename lookup files..."
TODOS_LOOKUP="/tmp/todos_lookup.txt"
ANIMATED_LOOKUP="/tmp/animated_lookup.txt"

# Build todos lookup file: normalized_name:original_filename
find "$ICONS_TODOS_DIR" -name "*.txt" -exec basename {} \; | while read -r filename; do
    normalized=$(normalize_filename "$filename")
    echo "$normalized:$filename"
done | sort > "$TODOS_LOOKUP"

# Build animated lookup file: normalized_name:original_filename
find "$ICONS_ANIMATED_DIR" -name "*.tsx" -exec basename {} \; | while read -r filename; do
    normalized=$(normalize_filename "$filename")
    echo "$normalized:$filename"
done | sort > "$ANIMATED_LOOKUP"

# Find files unique to each directory
comm -23 "$TODOS_NORMALIZED" "$ANIMATED_NORMALIZED" > "$TODOS_ONLY"
comm -13 "$TODOS_NORMALIZED" "$ANIMATED_NORMALIZED" > "$ANIMATED_ONLY"

# Count files
TODOS_COUNT=$(wc -l < "$TODOS_NORMALIZED")
ANIMATED_COUNT=$(wc -l < "$ANIMATED_NORMALIZED")
COMMON_COUNT=$(wc -l < "$COMMON_FILES")
TODOS_ONLY_COUNT=$(wc -l < "$TODOS_ONLY")
ANIMATED_ONLY_COUNT=$(wc -l < "$ANIMATED_ONLY")

# Calculate percentages
COMMON_PERCENTAGE_TODOS=$(( COMMON_COUNT * 100 / TODOS_COUNT ))
COMMON_PERCENTAGE_ANIMATED=$(( COMMON_COUNT * 100 / ANIMATED_COUNT ))

echo "📊 Generating markdown report..."

# Generate markdown report
cat > "$OUTPUT_FILE" << EOF
# Icon Files Comparison Report

**Generated on:** $(date '+%Y-%m-%d %H:%M:%S')

## Summary Statistics

| Metric | Count | Percentage |
|--------|-------|------------|
| **ICONS_TODOS** files (snake_case.txt) | $TODOS_COUNT | 100% |
| **icons-animated-pqoqubbw** files (kebab-case.tsx) | $ANIMATED_COUNT | 100% |
| **Common files** | $COMMON_COUNT | ${COMMON_PERCENTAGE_TODOS}% of TODOS, ${COMMON_PERCENTAGE_ANIMATED}% of animated |
| **TODOS only** | $TODOS_ONLY_COUNT | $(( TODOS_ONLY_COUNT * 100 / TODOS_COUNT ))% of TODOS |
| **Animated only** | $ANIMATED_ONLY_COUNT | $(( ANIMATED_ONLY_COUNT * 100 / ANIMATED_COUNT ))% of animated |

## Directory Information

- **ICONS_TODOS**: \`${ICONS_TODOS_DIR}\`
- **icons-animated-pqoqubbw**: \`${ICONS_ANIMATED_DIR}\`

## Naming Convention Examples

| ICONS_TODOS (snake_case) | icons-animated-pqoqubbw (kebab-case) | Normalized |
|--------------------------|-------------------------------------|------------|
EOF

# Add some example mappings (using lookup files)
head -5 "$COMMON_FILES" | while read -r normalized; do
    todos_original=$(grep "^$normalized:" "$TODOS_LOOKUP" | cut -d: -f2)
    animated_original=$(grep "^$normalized:" "$ANIMATED_LOOKUP" | cut -d: -f2)
    echo "| $todos_original | $animated_original | $normalized |" >> "$OUTPUT_FILE"
done

cat >> "$OUTPUT_FILE" << EOF

## Common Files ($COMMON_COUNT files)

The following files exist in both directories (after normalization):

| # | Icon Name | ICONS_TODOS File | icons-animated-pqoqubbw File |
|---|-----------|------------------|------------------------------|
EOF

# Add common files table
counter=1
while read -r normalized; do
    todos_original=$(grep "^$normalized:" "$TODOS_LOOKUP" | cut -d: -f2)
    animated_original=$(grep "^$normalized:" "$ANIMATED_LOOKUP" | cut -d: -f2)
    echo "| $counter | \`$normalized\` | $todos_original | $animated_original |" >> "$OUTPUT_FILE"
    counter=$((counter + 1))
done < "$COMMON_FILES"

# Add sections for unique files
cat >> "$OUTPUT_FILE" << EOF

## Files Only in ICONS_TODOS ($TODOS_ONLY_COUNT files)

These icons exist only in the ICONS_TODOS directory:

EOF

counter=1
while read -r normalized; do
    todos_original=$(grep "^$normalized:" "$TODOS_LOOKUP" | cut -d: -f2)
    echo "$counter. \`$normalized\` ($todos_original)" >> "$OUTPUT_FILE"
    counter=$((counter + 1))
    # Limit output to first 50 for readability
    if [ $counter -gt 50 ]; then
        echo "... and $((TODOS_ONLY_COUNT - 50)) more files" >> "$OUTPUT_FILE"
        break
    fi
done < "$TODOS_ONLY"

cat >> "$OUTPUT_FILE" << EOF

## Files Only in icons-animated-pqoqubbw ($ANIMATED_ONLY_COUNT files)

These icons exist only in the icons-animated-pqoqubbw directory:

EOF

counter=1
while read -r normalized; do
    animated_original=$(grep "^$normalized:" "$ANIMATED_LOOKUP" | cut -d: -f2)
    echo "$counter. \`$normalized\` ($animated_original)" >> "$OUTPUT_FILE"
    counter=$((counter + 1))
done < "$ANIMATED_ONLY"

cat >> "$OUTPUT_FILE" << EOF

## Analysis

- **Coverage**: $(echo "scale=1; $COMMON_COUNT * 100 / $ANIMATED_COUNT" | bc)% of animated icons have corresponding TODO files
- **Implementation Progress**: $COMMON_COUNT out of $ANIMATED_COUNT animated icons are tracked in TODOS
- **Remaining Work**: $ANIMATED_ONLY_COUNT animated icons don't have TODO entries
- **Potential New Icons**: $TODOS_ONLY_COUNT TODO items don't have animated implementations yet

---

*Report generated by compare_icons.sh*
EOF

# Cleanup temporary files
rm -f "$TODOS_NORMALIZED" "$ANIMATED_NORMALIZED" "$COMMON_FILES" "$TODOS_ONLY" "$ANIMATED_ONLY" "$TODOS_LOOKUP" "$ANIMATED_LOOKUP"

echo "✅ Report generated successfully: $OUTPUT_FILE"
echo ""
echo "📊 Quick Summary:"
echo "   - ICONS_TODOS: $TODOS_COUNT files"
echo "   - icons-animated-pqoqubbw: $ANIMATED_COUNT files"
echo "   - Common files: $COMMON_COUNT ($COMMON_PERCENTAGE_ANIMATED% coverage)"
echo "   - Report saved to: $OUTPUT_FILE"