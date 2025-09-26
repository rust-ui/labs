use leptos::prelude::*;
use crate::components::ui::tree_view::{Tree, Folder, FileRenderer};
use crate::components::ui::card::{Card, CardContent};

#[component]
pub fn DemoTreeViewShow() -> impl IntoView {
    view! {
        <div class="flex gap-4 w-full max-w-4xl">
            <Tree>
                <Folder name="src" open=true>
                    <FileRenderer name="main.rs" content="Main application entry point" checked=true />
                    <FileRenderer name="lib.rs" content="Library root module" />
                    <Folder name="components" open=true>
                        <Folder name="ui">
                            <FileRenderer name="button.rs" content=BUTTON_RS />
                            <FileRenderer name="card.rs" content=CARD_RS />
                            <FileRenderer name="input.rs" content=INPUT_RS />
                            <FileRenderer name="accordion.rs" content=ACCORDION_RS />
                        </Folder>
                        <FileRenderer name="mod.rs" content="Components module exports" />
                    </Folder>
                    <Folder name="utils">
                        <FileRenderer name="hooks.rs" content=HOOKS_RS />
                        <FileRenderer name="mod.rs" content="Utils module exports" />
                    </Folder>
                </Folder>

                <FileRenderer name="Cargo.toml" content="Rust package configuration" />
                <FileRenderer name="README.md" content="Project documentation" />
            </Tree>

            <Card class="flex-1">
                <CardContent>
                    <div id="content-display">
                        <div>
                            <h3 class="mb-2 font-semibold">main.rs</h3>
                            <pre class="text-sm bg-muted p-4 rounded-md overflow-x-auto"><code>Main application entry point</code></pre>
                        </div>
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */




/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

const BUTTON_RS: &str = r#"use leptos::prelude::*;
use leptos_ui::clx;

clx! {Button, button, "inline-flex items-center justify-center"}

#[component]
pub fn Button() -> impl IntoView {
    view! { <button>Click me</button> }
}"#;

const CARD_RS: &str = r#"use leptos::prelude::*;
use leptos_ui::clx;

clx! {Card, div, "rounded-lg border bg-card"}

#[component]
pub fn Card() -> impl IntoView {
    view! { <div class="card">Content</div> }
}"#;

const INPUT_RS: &str = r#"use leptos::prelude::*;
use leptos_ui::clx;

clx! {Input, input, "flex h-10 rounded-md border"}

#[component]
pub fn Input() -> impl IntoView {
    view! { <input type="text" /> }
}"#;

const ACCORDION_RS: &str = r#"use leptos::prelude::*;
use leptos_ui::clx;

clx! {Accordion, div, "w-full"}

#[component]
pub fn Accordion() -> impl IntoView {
    view! { <div>Accordion content</div> }
}"#;

const HOOKS_RS: &str = r#"use leptos::prelude::*;

pub fn use_random_id() -> String {
    format!("id_{}", js_sys::Math::random())
}

// Random hook utilities"#;