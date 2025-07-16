# Clean CSS

Clean CSS classes for the specified files: $ARGUMENTS

## Process

**Check progress marker `✔️` at top and skip completed steps**

⚠️ *IMPORTANT*: For each steps, you need to ommit your work.

For each commit, mark the top of the CSS file this pattern:
- `/* CLEAN: Step 1 ✔️ */`
- `/* CLEAN: Step 1 ✔️ Step 2 ✔️ */`
- `/* CLEAN: Step 1 ✔️ Step 2 ✔️ Step 3 ✔️ */` etc.



### Step 1: Keep JS classes target in memory (*COMMIT AFTER COMPLETION*)

If there is no adjacent JS file, you can skip this part.
Otherwise, make sure to list all the CSS classes used in the JS file and keep that in memory.
From that, do a mapping with adjacent HTML and CSS files.
Keep that in memory.
This way when you will change CSS classes you are sure not to break JS.


### Step 2: Flatten CSS if nested (*COMMIT AFTER COMPLETION*)

If the CSS is nested, flatten it. If not nested, you can skip this step.


### Step 3: Clean CSS with mainDiv (*COMMIT AFTER COMPLETION*)
1. Fix formatting (2-space indentation)
2. Replace `:root` CSS variables with direct values, remove `:root` block
3. Move `*` selector styles to `.mainDiv`, remove `*` selector
4. Move `html`/`body` styles to `.mainDiv`, remove empty selectors
5. Ensure root element has `class="mainDiv"` at the top



### Step 4: Replace hyphenated CSS classes (*COMMIT AFTER COMPLETION*)
1. Look at the adjacent .js (if any) and make sure not to brake JS by changing CSS class names.
2. Replace ALL hyphenated CSS classes with double underscores (e.g., `elg-table-wrap` → `elg__table__wrap`)
3. **IMPORTANT**: This includes ALL hyphens in class names - not just the first one (e.g., `looping-words__edge` → `looping__words__edge`)
4. Update both CSS selectors and HTML class references
5. Verify all references are updated consistently
6. **Double-check**: Search for any remaining hyphens in class names to ensure none were missed



### Step 5: Replace Generic CSS Class Names (*COMMIT AFTER COMPLETION*)
1. Identify generic class names like `.wrapper`, `.container`, `.item` that lack descriptive context
2. Replace with more descriptive names following the `__` convention (e.g., `.wrapper` → `.scroll__snap__container`)
3. Update both CSS selectors and HTML class references consistently
4. **IMPORTANT**: Also check and update adjacent JS file for any class references (querySelector, getElementsByClassName, etc.)
5. Update the CSS class usage report to reflect the new class names
6. Ensure new names clearly describe the component's purpose and functionality



### Step 6: Remove CSS Layers (*COMMIT AFTER COMPLETION*)
1. Remove all `@layer` declarations and their wrapper blocks
2. Keep the CSS rules but flatten them to the root level
3. Maintain proper indentation and structure
4. Ensure all CSS functionality remains intact



### Step 7: Remove Unused CSS Classes (*COMMIT AFTER COMPLETION*)
1. Check the adjacent HTML file for all CSS classes that are actually used
2. Compare with CSS classes defined in the CSS file
3. Remove any CSS classes that are defined in CSS but not used in HTML
4. Also check adjacent JS file (if any) to ensure classes used in JavaScript are not removed
5. Clean up any empty CSS rules after removal
6. Verify that all remaining CSS classes have corresponding usage in HTML or JS


