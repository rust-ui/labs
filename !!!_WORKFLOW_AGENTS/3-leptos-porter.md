## Workflow

1. Create `DemoComponentName` in `__ready_to_port__`.
2. Follow [*## Instructions*] and make sure it compiles with `cargo check`.
3. Launch server with `LEPTOS_SITE_ADDR="127.0.0.1:4002" LEPTOS_RELOAD_PORT="4003" cargo leptos watch`.
4. Meanwhile it compiles, launch `Live Server` of the HTML file.
5. Once Leptos has compiled, use `Playwright` to make sure Leptos implements well the HTML.



## Instructions

- **IMPORTANT**: For CSS `<style>`, **ALWAYS** wrap in `<style> {" "} </style>`.
- **IMPORTANT**: For JS `<script>`, **ALWAYS** wrap in `<script> {" "} </script>`.
- **IMPORTANT**: Make sure to **ALWAYS** use proper self-closing tags (ex: `<img />`, `<input />`, ...).
- **IMPORTANT**: Wrap symbols with quotes (ex: `<p>"▼"</p>`).


## Example

Here is what a typical port would look like:

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
2. **ALWAYS** create a git commit with a simple descriptive message:
   - Use format: `🤖 3 Leptos Port component_name`
   - Example: `🤖 3 Leptos Port carousel`
