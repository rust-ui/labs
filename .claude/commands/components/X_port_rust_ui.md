# Port to RUST-UI repo

Follow these instructions for the specified files: $ARGUMENTS

## INSTRUCTIONS

1. Read the data of the file. If Stylesheet or script, check also related files (if any).
2. Copy the content inside `../RUST-UI/app/src/registry/demos/demo_extensions`. If .css file or .js file, copy in `public/components/*`.
3. Add the component name in the `mod.rs`.
4. Add the components in `PageLabs`.
5. Run `cargo check` to make sure all is good.
6. In our current directory, **remove** the now unused demo (and `.js` + `.css` if any).
7. Commit your changes prepending with `✨ [PORT]: `.


