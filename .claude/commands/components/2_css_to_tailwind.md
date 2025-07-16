# Convert CSS to Tailwind

Convert CSS classes to Tailwind utilities for the specified files: $ARGUMENTS

## 🚨 IMPORTANT RESTRICTIONS

**DO NOT MODIFY JAVASCRIPT FILES OR ADD JAVASCRIPT CODE**
- Focus ONLY on CSS to Tailwind conversion
- Do NOT add JavaScript for dynamic Tailwind classes
- Do NOT modify or add `<script>` tags
- Keep all existing JavaScript functionality intact
- Let existing JavaScript handle dynamic behavior with CSS classes

## Workflow

1. **Find files**: Locate `.css` file and its ajacent `.html` file (OR `rs` file if it's on `src/`)
2. **Identify convertible classes**: Only convert CSS classes that match the patterns listed below
3. **Clean up CSS**: Remove the converted CSS rules from `.css` files
4. **Remove unused classes**: Remove any empty or unused CSS classes that have no content or are not referenced
5. **Update usage reports**: If a CSS class usage report exists, update it to reflect the changes

**Check progress marker `✔️` at top and skip completed steps**

⚠️ *IMPORTANT*: For each steps, you need to ommit your work.

For each commit, mark the top of the CSS file this pattern:
- `/* TW: Step 1 ✔️ */`
- `/* TW: Step 1 ✔️ Step 2 ✔️ */`
- `/* TW: Step 1 ✔️ Step 2 ✔️ Step 3 ✔️ */` etc.


## Convertible CSS Patterns

**Only convert CSS that can be replaced with these Tailwind utilities:**

### Step 1: Colors (*COMMIT AFTER COMPLETION*)
- **Text**: `text-red-500`, `text-blue-600`, `text-green-400`, etc.
- **Background**: `bg-gray-100`, `bg-blue-500`, `bg-red-200`, `bg-white/25`, `bg-black/50`, etc.  
- **Border**: `border-gray-300`, `border-blue-500`, etc.
- **Hex colors**: Use arbitrary syntax for custom hex values: `text-[#9e9e9e]`, `bg-[#f8f9fa]`, `border-[#dee2e6]`


### Step 2: Spacing (*COMMIT AFTER COMPLETION*)
- **Padding**: `p-4`, `p-8`, `px-6`, `py-2`, etc.
- **Margin**: `m-4`, `m-8`, `mx-auto`, `my-6`, etc.
- **Space between**: `space-y-4`, `space-x-2`, etc.
- **Custom measurements**: Use arbitrary syntax for em/rem units: `px-[.1em]`, `p-[1.5rem]`, `mt-[25%]`


### Step 3: Typography (*COMMIT AFTER COMPLETION*)
- **Font size**: `text-sm`, `text-lg`, `text-xl`, `text-2xl`, etc.
- **Font weight**: `font-bold`, `font-semibold`, `font-light`, etc.
- **Text align**: `text-center`, `text-left`, `text-right`, etc.
- **Font family**: `font-sans`, `font-serif`, `font-mono`, etc.
- **Custom sizes**: Use arbitrary syntax for specific measurements: `text-[1.125rem]`, `text-[1.2rem]`
- **Line height**: Use arbitrary syntax for unitless values: `leading-[.9]`, `leading-[1.5]`


### Step 4: Layout (*COMMIT AFTER COMPLETION*)
- **Display**: `block`, `inline-block`, `inline`, `flex`, `inline-flex`, `grid`, `hidden`, etc.
- **Visibility**: `visible`, `invisible`, etc.
- **Flexbox**:  `flex`, `justify-center`, `justify-between`, `items-center`, `flex-col`, `flex-row`, etc.
- **Position**: `static`, `relative`, `absolute`, `fixed`, `sticky`, etc.
- **Inset**: `inset-0`, `top-0`, `right-0`, `bottom-0`, `left-0`, etc.
- **Custom positioning**: Use arbitrary syntax for specific pixel values: `top-[19px]`, `right-[296px]`, `bottom-[16px]`
- **Z-index**: `z-0`, `z-10`, `z-20`, `z-30`, `z-40`, `z-50`, etc.
- **Box sizing**: `box-border`, `box-content`, etc.
- **Complex inset**: Use arbitrary syntax for complex values: `inset-[auto_0_0_auto]`, `inset-[10px_20px]`


### Step 5: Sizing (*COMMIT AFTER COMPLETION*)
- **Width**: `w-full`, `w-1/2`, `w-64`, `w-96`, `w-[75px]`, etc.
- **Max width**: `max-w-md`, `max-w-lg`, `max-w-xl`, `max-w-2xl`, etc.
- **Height**: `h-full`, `h-64`, `h-96`, `h-screen`, etc.
- **Max height**: `max-h-64`, `max-h-96`, `max-h-screen`, etc.
- **Min width**: `min-w-0`, `min-w-full`, etc.
- **Min height**: `min-h-0`, `min-h-full`, `min-h-screen`, etc.
- **Custom sizes**: Use arbitrary syntax for specific measurements: `min-w-[25rem]`, `h-[2.7em]`, `w-[140%]`
- **Custom pixel sizes**: Use arbitrary syntax for specific pixel values: `w-[344px]`, `h-[8px]`, `w-[12px]`


### Step 6: Interactivity & Visual (*COMMIT AFTER COMPLETION*)
- **Cursor**: `cursor-pointer`, `cursor-default`, `cursor-wait`, etc.
- **Background**: `bg-transparent`, `bg-opacity-50`, etc.
- **Background none**: `background: none` → `bg-transparent`
- **Border none**: `border: none` → `border-0`
- **Border radius**: `rounded`, `rounded-lg`, `rounded-full`, `rounded-2xl`, `rounded-[1.2rem]`, etc.
- **Overflow**: `overflow-hidden`, `overflow-visible`, `overflow-scroll`, etc.
- **List style**: `list-none`, `list-disc`, `list-decimal`, etc.
- **Transform**: ⚠️ **DO NOT CONVERT** scale transforms - keep `transform: scale()` in CSS for animations
- **Fill**: `fill-white`, `fill-black`, `fill-gray-500`, `fill-blue-600`, etc.
- **Complex borders**: Use arbitrary syntax for custom borders: `border-t-[.035em] border-solid border-[#007bff]`
- **Keep complex CSS**: Don't convert animations, transforms, or custom properties - preserve these as CSS

## ⚠️ PRESERVE DYNAMIC CSS CLASSES

**DO NOT REMOVE CSS classes that are dynamically applied by JavaScript:**
- State-dependent classes (e.g., `.active`, `.valid`, `.invalid`)
- Classes applied based on user interaction
- Classes that change based on data or conditions
- Strength levels, validation states, or any CSS managed by existing JavaScript

**When in doubt, KEEP the CSS class** - it's better to have redundant CSS than broken functionality.



## Additional Conversion Notes

- **@keyframes naming**: Always prefix keyframe names with component/feature name using __ pattern:
  - `@keyframes scale` → `@keyframes scroll__snap__scale`
  - `@keyframes fade` → `@keyframes modal__fade`
- **Separate commits**: Create dedicated commits for arbitrary value conversions:
  - `refactor: convert [component_name] hardcoded values to Tailwind arbitrary syntax.`
- **Active state classes**: When encountering `.active` classes or state-dependent styles, add "/* Active class */" comments above each selector to indicate active state styling
