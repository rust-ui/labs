# Convert CSS to Tailwind

Convert CSS classes to Tailwind utilities for the specified files: $ARGUMENTS

## Workflow

1. **Find files**: Locate `.css` file and its ajacent `.html` file (OR `rs` file if it's on `src/`)
2. **Identify convertible classes**: Only convert CSS classes that match the patterns listed below
3. **Clean up CSS**: Remove the converted CSS rules from `.css` files
4. **Remove unused classes**: Remove any empty or unused CSS classes that have no content or are not referenced
5. **Update usage reports**: If a CSS class usage report exists, update it to reflect the changes


## Convertible CSS Patterns

**Only convert CSS that can be replaced with these Tailwind utilities:**

### Colors
- **Text**: `text-red-500`, `text-blue-600`, `text-green-400`, etc.
- **Background**: `bg-gray-100`, `bg-blue-500`, `bg-red-200`, `bg-white/25`, `bg-black/50`, etc.  
- **Border**: `border-gray-300`, `border-blue-500`, etc.

### Spacing
- **Padding**: `p-4`, `p-8`, `px-6`, `py-2`, etc.
- **Margin**: `m-4`, `m-8`, `mx-auto`, `my-6`, etc.
- **Space between**: `space-y-4`, `space-x-2`, etc.

### Typography  
- **Font size**: `text-sm`, `text-lg`, `text-xl`, `text-2xl`, etc.
- **Font weight**: `font-bold`, `font-semibold`, `font-light`, etc.
- **Text align**: `text-center`, `text-left`, `text-right`, etc.
- **Font family**: `font-sans`, `font-serif`, `font-mono`, etc.

### Layout
- **Display**: `block`, `inline-block`, `inline`, `flex`, `inline-flex`, `grid`, `hidden`, etc.
- **Flexbox**: `justify-center`, `justify-between`, `items-center`, etc.
- **Position**: `static`, `relative`, `absolute`, `fixed`, `sticky`, etc.
- **Inset**: `inset-0`, `top-0`, `right-0`, `bottom-0`, `left-0`, etc.
- **Z-index**: `z-0`, `z-10`, `z-20`, `z-30`, `z-40`, `z-50`, etc.
- **Box sizing**: `box-border`, `box-content`, etc.

### Sizing
- **Width**: `w-full`, `w-1/2`, `w-64`, `w-96`, `w-[75px]`, etc.
- **Max width**: `max-w-md`, `max-w-lg`, `max-w-xl`, `max-w-2xl`, etc.
- **Height**: `h-full`, `h-64`, `h-96`, `h-screen`, etc.
- **Max height**: `max-h-64`, `max-h-96`, `max-h-screen`, etc.
- **Min width**: `min-w-0`, `min-w-full`, etc.
- **Min height**: `min-h-0`, `min-h-full`, `min-h-screen`, etc.

### Interactivity & Visual
- **Cursor**: `cursor-pointer`, `cursor-default`, `cursor-wait`, etc.
- **Background**: `bg-transparent`, `bg-opacity-50`, etc.
- **Border radius**: `rounded`, `rounded-lg`, `rounded-full`, `rounded-2xl`, `rounded-[1.2rem]`, etc.
- **Overflow**: `overflow-hidden`, `overflow-visible`, `overflow-scroll`, etc.
- **Transform**: `scale-95`, `scale-100`, `scale-105`, `scale-110`, etc.
- **Fill**: `fill-white`, `fill-black`, `fill-gray-500`, `fill-blue-600`, etc.



## Important Notes

- **Keep complex CSS**: Don't convert complex animations, transforms, or custom properties
- **Use specific naming for @keyframes**: Always prefix keyframe names with the component/feature name using __ pattern to avoid conflicts
  - Example: `@keyframes scale` → `@keyframes scroll__snap__scale`
  - Example: `@keyframes fade` → `@keyframes modal__fade`
  - Example: `@keyframes slide` → `@keyframes carousel__slide`
- **Preserve hardcoded values**: When CSS uses specific measurements that make sense to preserve, use Tailwind's arbitrary value syntax
  - Example: `min-width: 25rem` → `min-w-[25rem]` 
  - Example: `padding: 1.5rem` → `p-[1.5rem]`
  - Example: `font-size: 1.125rem` → `text-[1.125rem]`
- **Create separate commits**: If hardcoded values are converted to arbitrary value syntax, make a dedicated commit specifically for those conversions:
  - `refactor: convert [component_name] hardcoded values to Tailwind arbitrary syntax.`
- **Convert em/rem units**: When CSS uses em or rem units, convert them to Tailwind arbitrary value syntax to preserve the exact measurements:
  - Example: `padding-left: .1em; padding-right: .1em;` → `px-[.1em]`
  - Example: `height: 2.7em` → `h-[2.7em]`
  - Example: `height: .9em` → `h-[.9em]`
  - Example: `width: .125em; height: .125em;` → `w-[.125em] h-[.125em]`
  - Example: `font-size: 1.2rem` → `text-[1.2rem]`
- **Convert unitless line-height**: When CSS uses unitless line-height values, convert them to Tailwind arbitrary value syntax:
  - Example: `line-height: .9` → `leading-[.9]`
  - Example: `line-height: 1.5` → `leading-[1.5]`
- **Convert hex colors**: When CSS uses hex color values that don't match standard Tailwind colors, convert them to Tailwind arbitrary value syntax:
  - Example: `color: #9e9e9e` → `text-[#9e9e9e]` (in HTML)
  - Example: `background-color: #f8f9fa` → `bg-[#f8f9fa]` (in HTML)
  - Example: `border-color: #dee2e6` → `border-[#dee2e6]` (in HTML)
- **Convert percentage values**: When CSS uses percentage values that don't match standard Tailwind utilities, convert them to Tailwind arbitrary value syntax:
  - Example: `width: 140%` → `w-[140%]` (in HTML)
  - Example: `left: -20%` → `left-[-20%]` (in HTML)
  - Example: `margin-top: 25%` → `mt-[25%]` (in HTML)
- **Convert border properties**: When CSS uses border properties with custom values, convert them to Tailwind arbitrary value syntax:
  - Example: `border-top: .035em solid #007bff; border-left: .035em solid #007bff;` → `border-t-[.035em] border-l-[.035em] border-solid border-[#007bff]` (in HTML)
  - Example: `border: 2px dashed #ff0000` → `border-[2px] border-dashed border-[#ff0000]` (in HTML)
  - Example: `border-radius: 1.5rem` → `rounded-[1.5rem]` (in HTML)
