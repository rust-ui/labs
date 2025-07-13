# Convert CSS to Tailwind

Convert CSS classes to Tailwind utilities for the specified files: $ARGUMENTS

## Workflow

1. **Find files**: Locate `.css` files in `public/components/` and corresponding `.rs` files in `src/__demos__/`
2. **Identify convertible classes**: Only convert CSS classes that match the patterns listed below
3. **Convert in Rust files**: Replace custom CSS classes with Tailwind classes in `clx!` macros
4. **Clean up CSS**: Remove the converted CSS rules from `.css` files
5. **Test**: Verify the component still works correctly


## Convertible CSS Patterns

**Only convert CSS that can be replaced with these Tailwind utilities:**

### Colors
- **Text**: `text-red-500`, `text-blue-600`, `text-green-400`, etc.
- **Background**: `bg-gray-100`, `bg-blue-500`, `bg-red-200`, etc.  
- **Border**: `border-gray-300`, `border-blue-500`, etc.

### Spacing
- **Padding**: `p-4`, `p-8`, `px-6`, `py-2`, etc.
- **Margin**: `m-4`, `m-8`, `mx-auto`, `my-6`, etc.
- **Space between**: `space-y-4`, `space-x-2`, etc.

### Typography  
- **Font size**: `text-sm`, `text-lg`, `text-xl`, `text-2xl`, etc.
- **Font weight**: `font-bold`, `font-semibold`, `font-light`, etc.
- **Text align**: `text-center`, `text-left`, `text-right`, etc.

### Layout
- **Display**: `block`, `inline-block`, `inline`, `flex`, `inline-flex`, `grid`, `hidden`, etc.
- **Flexbox**: `justify-center`, `justify-between`, `items-center`, etc.
- **Position**: `static`, `relative`, `absolute`, `fixed`, `sticky`, etc.
- **Inset**: `inset-0`, `top-0`, `right-0`, `bottom-0`, `left-0`, etc.
- **Z-index**: `z-0`, `z-10`, `z-20`, `z-30`, `z-40`, `z-50`, etc.

### Sizing
- **Width**: `w-full`, `w-1/2`, `w-64`, `w-96`, etc.
- **Max width**: `max-w-md`, `max-w-lg`, `max-w-xl`, `max-w-2xl`, etc.
- **Height**: `h-full`, `h-64`, `h-96`, `h-screen`, etc.
- **Max height**: `max-h-64`, `max-h-96`, `max-h-screen`, etc.
- **Min width**: `min-w-0`, `min-w-full`, etc.
- **Min height**: `min-h-0`, `min-h-full`, `min-h-screen`, etc.

### Interactivity & Visual
- **Cursor**: `cursor-pointer`, `cursor-default`, `cursor-wait`, etc.
- **Background**: `bg-transparent`, `bg-opacity-50`, etc.
- **Border radius**: `rounded`, `rounded-lg`, `rounded-full`, `rounded-2xl`, etc.
- **Overflow**: `overflow-hidden`, `overflow-visible`, `overflow-scroll`, etc.



## Important Notes

- **Keep complex CSS**: Don't convert complex animations, transforms, or custom properties
