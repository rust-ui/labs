## Instructions

- **IMPORTANT**: Do NOT remove Tailwind CDN.


**Flatten CSS:**
- If the CSS is nested, flatten it. If not nested, you can skip this step.
- Check if there is extra spaces useless. If any, remvove them.
- We must have an empty line between each classes.



**Clean CSS with mainDiv:**
1. Fix formatting (2-space indentation)
2. Replace `:root` CSS variables with direct values, remove `:root` block
3. Move `*` selector styles to `.mainDiv`, remove `*` selector
4. Move `html`/`body` styles to `.mainDiv`, remove empty selectors. `.mainDiv` SHOULD NOT be in `body` but in a regular `div`. 
5. Ensure root element has `class="mainDiv"` at the top
6. **IMPORTANT**: Do NOT add `font-family` to `.mainDiv` - let the system use default fonts


**Others**
- Remove all `@layer` declarations and their wrapper blocks
- Identify generic class names like `.wrapper`, `.container`, `.item` that lack descriptive context
- Replace with more descriptive names following the `__` convention (e.g., `.wrapper` → `.scroll__snap__container`)
- **JS Class Naming**: Inside JS `<script>`, replace generic state classes with meaningful names using `__` convention:
  - `.open` → `.open__split__button`
  - `.active` → `.active__modal`
  - `.show` → `.show__dropdown`
  - Update both JS and CSS accordingly.



## Task Completion

Upon task completion:
1. **ALWAYS** run `~/.claude/sound_task_complete.sh`
2. **ALWAYS** create a git commit with a simple descriptive message:
   - Use format: `🤖 1 CSS Clean component_name`
   - Example: `🤖 1 CSS Clean carousel`
