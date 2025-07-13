# Clean CSS

Clean CSS classes for the specified files: $ARGUMENTS

## Process

### Step 1: Clean CSS with mainDiv (COMMIT AFTER COMPLETION)
1. Fix formatting (2-space indentation)
2. Replace `:root` CSS variables with direct values, remove `:root` block
3. Move `*` selector styles to `.mainDiv`, remove `*` selector
4. Move `html`/`body` styles to `.mainDiv`, remove empty selectors
5. Ensure root element has `class="mainDiv"` at the top

Note: If it's already clean, comment at top: `/* Clean CSS: No :root variables, no * selector, no html/body selectors */`


### Step 2: Replace hyphenated CSS classes (COMMIT AFTER COMPLETION)
1. Replace hyphenated CSS classes with double underscores (e.g., `elg-table-wrap` → `elg__table__wrap`)
2. Update both CSS selectors and HTML class references
3. Verify all references are updated consistently


