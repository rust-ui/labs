# Clean CSS

Clean CSS classes for the specified files: $ARGUMENTS

## Process

**Check progress marker `✔️` at top and skip completed steps**

### Step 1: Clean CSS with mainDiv (COMMIT AFTER COMPLETION)
1. Fix formatting (2-space indentation)
2. Replace `:root` CSS variables with direct values, remove `:root` block
3. Move `*` selector styles to `.mainDiv`, remove `*` selector
4. Move `html`/`body` styles to `.mainDiv`, remove empty selectors
5. Ensure root element has `class="mainDiv"` at the top

Note: When complete, add progress marker at top: `/* Step 1 ✔️ */`


### Step 2: Replace hyphenated CSS classes (COMMIT AFTER COMPLETION)
1. Replace hyphenated CSS classes with double underscores (e.g., `elg-table-wrap` → `elg__table__wrap`)
2. Update both CSS selectors and HTML class references
3. Verify all references are updated consistently

Note: When complete, update progress marker: `/* Step 1 ✔️ Step 2 ✔️ */`


### Step 3: Add CSS Class Usage Report (COMMIT AFTER COMPLETION)
1. Count the number of times each CSS class is used in the HTML file
2. Add a usage report comment at the top of the CSS file, just before the first class definition:
```css
/*
====== CSS CLASS USAGE REPORT ======

.elg__table__wrap     → 1 time
.elg__table           → 1 time
.elg__table__hint     → 1 time

*/
```
3. Format each class name with arrow and usage count
4. Place report after any existing header comments but before CSS rules

Note: When complete, update progress marker: `/* Step 1 ✔️ Step 2 ✔️ Step 3 ✔️ */`


