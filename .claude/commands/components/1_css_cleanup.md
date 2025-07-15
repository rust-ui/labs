# Clean CSS

Clean CSS classes for the specified files: $ARGUMENTS

## Process

**Check progress marker `✔️` at top and skip completed steps**

⚠️ *IMPORTANT*: For each steps, ask me the permission to continue. This helps me to make sure all works properly.

### Step 1: Understand the CSS / HTML / (JS)

Make sure to understand first the files and adjacents (CSS / HTML / possibly JS).
Keep the context in memory.


### Step 2: Clean CSS with mainDiv (COMMIT AFTER COMPLETION)
1. Fix formatting (2-space indentation)
2. Replace `:root` CSS variables with direct values, remove `:root` block
3. Move `*` selector styles to `.mainDiv`, remove `*` selector
4. Move `html`/`body` styles to `.mainDiv`, remove empty selectors
5. Ensure root element has `class="mainDiv"` at the top

Note: When complete, add progress marker at top: `/* Step 1 ✔️ */`


### Step 3: Replace hyphenated CSS classes (COMMIT AFTER COMPLETION)
1. Look at the adjacent .js (if any) and make sure not to brake JS by changing CSS class names.
2. Replace hyphenated CSS classes with double underscores (e.g., `elg-table-wrap` → `elg__table__wrap`)
3. Update both CSS selectors and HTML class references
4. Verify all references are updated consistently

Note: When complete, update progress marker: `/* Step 1 ✔️ Step 2 ✔️ */`


### Step 4: Replace Generic CSS Class Names (COMMIT AFTER COMPLETION)
1. Identify generic class names like `.wrapper`, `.container`, `.item` that lack descriptive context
2. Replace with more descriptive names following the `__` convention (e.g., `.wrapper` → `.scroll__snap__container`)
3. Update both CSS selectors and HTML class references consistently
4. Update the CSS class usage report to reflect the new class names
5. Ensure new names clearly describe the component's purpose and functionality

Note: When complete, update progress marker: `/* Step 1 ✔️ Step 2 ✔️ Step 3 ✔️ */`


### Step 5: Remove CSS Layers (COMMIT AFTER COMPLETION)
1. Remove all `@layer` declarations and their wrapper blocks
2. Keep the CSS rules but flatten them to the root level
3. Maintain proper indentation and structure
4. Ensure all CSS functionality remains intact

Note: When complete, update progress marker: `/* Step 1 ✔️ Step 2 ✔️ Step 3 ✔️ Step 4 ✔️ */`


