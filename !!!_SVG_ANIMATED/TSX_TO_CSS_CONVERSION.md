# TSX to CSS Conversion

1. Read `.tsx` file for animation variants and `.txt` file for SVG paths
2. **SVG animations** → `[data-name="IconAnimate"]:hover svg`
3. **Path animations** → `[data-name="IconAnimate"]:hover path[d="exact-path"]`
4. Convert TSX timing: `duration: 0.5` → CSS `0.5s`, `delay: 0.2` → `0.2s`
5. Convert transforms: `scale: [1, 1.2, 1]` → CSS keyframes
6. Add trailing newline and test in `SVG-ANIMATED.html`