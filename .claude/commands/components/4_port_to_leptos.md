# Convert CSS to Tailwind

Convert CSS classes to Tailwind CSS for the specified files: $ARGUMENTS

## Workflow

1. **Find files**: Locate `.css` files in `public/components/` and corresponding `.rs` files in `src/__demos__/`
2. **Identify convertible classes**: Only convert CSS classes that match the patterns listed below
3. **Convert in Rust files**: Replace custom CSS classes with Tailwind classes in `clx!` macros
4. **Clean up CSS**: Remove the converted CSS rules from `.css` files
5. **Test**: Run `cargo check` to make sure there is no error.


// TODO. 
- sanitize self closing elements like img, inputs, etc. to fit with Leptos.
