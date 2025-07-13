# Clean CSS

Clean CSS classes for the specified files: $ARGUMENTS

## Process
1. Fix formatting (2-space indentation) - separate commit if needed
2. Replace `:root` CSS variables with direct values, remove `:root` block
3. Move `*` selector styles to `.mainDiv`, remove `*` selector
4. Move `html`/`body` styles to `.mainDiv`, remove empty selectors
5. Ensure root element has `class="mainDiv"` at the top

## Rules
- Remove all `:root`, `*`, `html`, `body` selectors
- Move all their styles to `.mainDiv` class
- Replace `var(--name)` with actual values
- Keep 2-space CSS indentation
- Add clean CSS rule comment at top: `/* Clean CSS: No :root variables, no * selector, no html/body selectors */`, ONLY when it's truly the case. 💁


