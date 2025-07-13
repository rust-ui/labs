# Clean CSS

Clean CSS classes for the specified files: $ARGUMENTS

## Rules
- Remove CSS variables from `:root` and replace with direct color values in properties that use them
- Move all styles from `html` and `body` selectors to `.mainDiv` class, create corresponding HTML wrapper, then remove empty `html`/`body` rules
- If universal selector `* { box-sizing: border-box; }` exists, also add `box-sizing: border-box;` to `.mainDiv` and remove the universal selector

## Formatting
- Clean up inconsistent indentation to use 2-space indentation throughout

