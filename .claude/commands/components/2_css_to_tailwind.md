# Convert CSS to Tailwind

Convert CSS classes to Tailwind CSS for the specified files: $ARGUMENTS

## 🚨 IMPORTANT RESTRICTIONS

**DO NOT MODIFY JAVASCRIPT FILES OR ADD JAVASCRIPT CODE**
- Focus ONLY on CSS to Tailwind conversion
- Do NOT add JavaScript for dynamic Tailwind classes
- Do NOT modify or add `<script>` tags
- Keep all existing JavaScript functionality intact
- Let existing JavaScript handle dynamic behavior with CSS classes

## Workflow

1. **Find files**: Locate `.css` file and its ajacent `.html` file (OR `rs` file if it's on `src/`)
2. **🚨 CRITICAL: Check JavaScript files first**: 
   - Read ALL `.js` files in the project to understand dynamic class usage
   - Create a list of CSS classes that are added/removed/toggled by JavaScript
   - **NEVER convert CSS classes that are dynamically used by JavaScript** (e.g., `.active`, `.open`, `.visible`, `.selected`, etc.)
   - Look for patterns like `classList.add()`, `classList.remove()`, `classList.toggle()`, `className =`, etc.
3. **Identify convertible classes**: Only convert CSS classes that match the patterns listed below AND are NOT used by JavaScript
4. **Clean up CSS**: Remove the converted CSS rules from `.css` files
5. **Remove unused classes**: Remove any empty or unused CSS classes that have no content or are not referenced
6. **Update usage reports**: If a CSS class usage report exists, update it to reflect the changes

**Check progress marker `✔️` at top and skip completed steps**

⚠️ *IMPORTANT*: For each steps, you need to ommit your work.

For each commit, mark the top of the CSS file this pattern:
- `/* TW: Step 1 ✔️ */`
- `/* TW: Step 1 ✔️ Step 2 ✔️ */`
- `/* TW: Step 1 ✔️ Step 2 ✔️ Step 3 ✔️ */` etc.

⚠️ **IMPORTANT**: If a step has no convertible changes (e.g., no typography to convert), do NOT create a commit just to update the progress marker. Simply update the marker and proceed directly to the next step with actual changes.


## Convertible CSS Patterns

**Only convert CSS that can be replaced with these Tailwind CSS:**

### Step 1: Spacing (*COMMIT AFTER COMPLETION*)
- **Padding**: `p-4`, `p-8`, `px-6`, `py-2`, etc.
- **Margin**: `m-4`, `m-8`, `mx-auto`, `my-6`, etc.
- **Space between**: `space-y-4`, `space-x-2`, etc.
- **Custom measurements**: Use arbitrary syntax for em/rem units: `px-[.1em]`, `p-[1.5rem]`, `mt-[25%]`


### Step 2: Typography (*COMMIT AFTER COMPLETION*)
- **Font size**: `text-sm`, `text-lg`, `text-xl`, `text-2xl`, etc.
- **Font weight**: `font-bold`, `font-semibold`, `font-light`, etc.
- **Text align**: `text-center`, `text-left`, `text-right`, etc.
- **Font family**: `font-sans`, `font-serif`, `font-mono`, etc. (Use only Tailwind built-in fonts, remove custom font imports)
- **Letter spacing**: `tracking-tight`, `tracking-normal`, `tracking-wide`, etc.
- **Custom sizes**: Use arbitrary syntax for specific measurements: `text-[1.125rem]`, `text-[1.2rem]`
- **Line height**: Use arbitrary syntax for unitless values: `leading-[.9]`, `leading-[1.5]`
- **Custom letter spacing**: Use arbitrary syntax for specific values: `tracking-[0.01em]`, `tracking-[0.03em]`


### Step 3: Interactivity & Visual (*COMMIT AFTER COMPLETION*)
- **Cursor**: `cursor-pointer`, `cursor-default`, `cursor-wait`, etc.
- **Background**: `bg-transparent`, `bg-opacity-50`, etc.
- **Background none**: `background: none` → `bg-transparent`
- **Border none**: `border: none` → `border-0`
- **Outline none**: `outline: none` → `outline-none`
- **Border radius**: `rounded`, `rounded-lg`, `rounded-full`, `rounded-2xl`, `rounded-[1.2rem]`, etc.
- **Overflow**: `overflow-hidden`, `overflow-visible`, `overflow-scroll`, etc.
- **List style**: `list-none`, `list-disc`, `list-decimal`, etc.
- **Transform**: ⚠️ **DO NOT CONVERT** scale transforms - keep `transform: scale()` in CSS for animations
- **Fill**: `fill-white`, `fill-black`, `fill-gray-500`, `fill-blue-600`, etc.
- **Complex borders**: Use arbitrary syntax for custom borders: `border-t-[.035em] border-solid border-[#007bff]`
- **Keep complex CSS**: Don't convert animations, transforms, or custom properties - preserve these as CSS


### Step 4: Layout (*COMMIT AFTER COMPLETION*)
- **Display**: `block`, `inline-block`, `inline`, `flex`, `inline-flex`, `grid`, `hidden`, etc.
- **Visibility**: `visible`, `invisible`, etc.
- **Flexbox**:  `flex`, `justify-center`, `justify-between`, `items-center`, `flex-col`, `flex-row`, etc.
- **Position**: `static`, `relative`, `absolute`, `fixed`, `sticky`, etc.
- **Inset**: `inset-0`, `top-0`, `right-0`, `bottom-0`, `left-0`, etc.
- **Custom positioning**: Use arbitrary syntax for specific pixel values: `top-[19px]`, `right-[296px]`, `bottom-[16px]`, `top-[48%]`, `bottom-[5%]`, `left-[50%]`
- **Z-index**: `z-0`, `z-10`, `z-20`, `z-30`, `z-40`, `z-50`, etc.
- **Box sizing**: `box-border`, `box-content`, etc.
- **Complex inset**: Use arbitrary syntax for complex values: `inset-[auto_0_0_auto]`, `inset-[10px_20px]`
- **Borders**: `border-solid`, `border-dashed`, `border-dotted`, `border-double`, `border-none`, etc.
- **Border widths**: Use arbitrary syntax for specific border widths: `border-[4px]`, `border-r-[4px]`, `border-b-[4px]`, `border-l-0`, `border-t-0`
- **Border combinations**: Convert `border: solid black; border-width: 0 4px 4px 0` to `border-solid border-black border-r-[4px] border-b-[4px] border-l-0 border-t-0`


### Step 5: Sizing (*COMMIT AFTER COMPLETION*)
- **Width**: `w-full`, `w-1/2`, `w-64`, `w-96`, `w-[75px]`, etc.
- **Max width**: `max-w-md`, `max-w-lg`, `max-w-xl`, `max-w-2xl`, etc.
- **Height**: `h-full`, `h-64`, `h-96`, `h-screen`, etc.
- **Max height**: `max-h-64`, `max-h-96`, `max-h-screen`, etc.
- **Min width**: `min-w-0`, `min-w-full`, etc.
- **Min height**: `min-h-0`, `min-h-full`, `min-h-screen`, etc.
- **Custom sizes**: Use arbitrary syntax for specific measurements: `min-w-[25rem]`, `h-[2.7em]`, `w-[140%]`
- **Custom pixel sizes**: Use arbitrary syntax for specific pixel values: `w-[344px]`, `h-[8px]`, `w-[12px]`


### Step 6: Colors (*COMMIT AFTER COMPLETION*)
- **Text**: `text-red-500`, `text-blue-600`, `text-green-400`, etc.
- **Background**: `bg-gray-100`, `bg-blue-500`, `bg-red-200`, `bg-white/25`, `bg-black/50`, etc.  
- **Border**: `border-gray-300`, `border-blue-500`, etc.
- **Border with width**: `border: 1px solid #color` → `border border-[#color]`
- **Hex colors**: Use arbitrary syntax for custom hex values: `text-[#9e9e9e]`, `bg-[#f8f9fa]`, `border-[#dee2e6]`



### Final Step: Remove any unused class (*COMMIT AFTER COMPLETION*)
Remove any unused classes from both CSS and HTML.



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
- **Active state classes**: When encountering `.active` classes or state-dependent styles, add "/* Active class */" comments above each selector to indicate active state styling
- **No conversion comments**: Do NOT add comments for CSS properties that you remove during conversion. Simply remove the CSS properties and add the Tailwind classes to HTML without explanatory comments.
