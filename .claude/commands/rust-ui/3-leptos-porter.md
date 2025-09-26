---
name: Unified Rust/UI Porter
description: Complete workflow from HTML component to Rust/UI registry
# allowed-tools:
argument-hint: ["html_location"]
---


## Input

1. *$html_location*: HTML file with component to port (component name will be inferred from filename)


## Workflow

### Phase 1: HTML to Leptos Port

1. Make sure you have *$html_location*. If not, ask explicitly to user.
2. Infer component name from *$html_location* (e.g., TREE-VIEW.html → DemoTreeView).
3. Create `Demo$component_name` in `__ready_to_port__/`.
3. Import in `all_demos.rs`.
4. Follow [*## Leptos Instructions*] and make sure it compiles with `cargo check`.
5. Launch server with `LEPTOS_SITE_ADDR="127.0.0.1:4002" LEPTOS_RELOAD_PORT="4003" cargo leptos watch`.
6. Meanwhile it compiles, launch `Live Server` of the HTML file.
7. Once Leptos has compiled, use `Playwright` to make sure Leptos implements well the HTML.
8. Create git commit with format: `🤖 3 Leptos Port $component_name`

### Phase 2: Leptos to Rust/UI Registry Port

9. Make sure you have *$registry_type*. If not, ask explicitly to user.
10. Go to `../RUST-UI/app/src/registry/`.
11. Put the `Demo$component_name` inside `registry/demos/`. Don't forget the `mod.rs`.
12. If `Demo$component_name` contains functions outside of the `Demo*`, put them in `registry/$registry_type`.
    1. Don't forget the `mod.rs`.
    2. Don't forget to bring to scope the components inside `Demo$component_name`.
13. Go inside `RUST-UI/public/docs/$registry_type`.
    1. Check if a `.md` file already exist. If not, create it.
    2. Add the documentation following the existing patterns.
14. Go inside `RUST-UI/build_registry/` and run `cargo run`.
15. Create git commit with format: `🤖 5 Registry Port $component_name`


## Leptos Instructions

- **IMPORTANT**: For CSS `<style>`, **ALWAYS** wrap in `<style> {" "} </style>`.
- **IMPORTANT**: For JS `<script>`, **ALWAYS** wrap in `<script> {" "} </script>`.
- **IMPORTANT**: Make sure to **ALWAYS** use proper self-closing tags (ex: `<img />`, `<input />`, ...).
- **IMPORTANT**: Wrap text content with quotes (ex: `<p>"▼"</p>`, `<span>"Text content"</span>`).


## Example

Here is what a typical Leptos port would look like:

```rust
use leptos::prelude::*;

#[component]
pub fn DemoComponentName() -> impl IntoView {
    view! {
       <style>
        {"
            // CSS goes here.
        "}
       </style>


       <div>
            // HTML elements goes here (with Leptos syntax).
       </div>


        <script>
        {"
            // JS goes here.
        "}
       </script>
    }
}
```

## Task Completion

Upon task completion:
1. **ALWAYS** run `~/.claude/sound_task_complete.sh`
2. Component should be fully functional in both Leptos demo and Rust/UI registry
3. All build registry outputs should be generated successfully


## Output

A fully functional component that has been:
1. Ported from HTML to Leptos format
2. Tested and verified to match original HTML behavior
3. Integrated into the RUST-UI registry with proper documentation
4. Built successfully with all registry outputs generated