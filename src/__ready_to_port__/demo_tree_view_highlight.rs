use leptos::prelude::*;

use crate::components::ui::card::{Card, CardContent};
use crate::components::ui::tree_view::{FileRenderer, Folder, Tree};
use crate::utils::syntax_highlighting_syntect::highlight_code;

#[component]
pub fn DemoTreeViewHighlight() -> impl IntoView {
    view! {
        <div class="flex gap-4 w-full max-w-4xl">
            <Tree>
                <Folder name="src" open=true>
                    <FileRenderer
                        name="main.rs"
                        content=MAIN_RS
                        checked=true
                        language="rust"
                    />
                    <FileRenderer name="lib.rs" content="Library root module" language="rust" />
                    <Folder name="components" open=true>
                        <Folder name="ui">
                            <FileRenderer name="button.rs" content=BUTTON_RS language="rust" />
                            <FileRenderer name="card.rs" content=CARD_RS language="rust" />
                            <FileRenderer name="input.rs" content=INPUT_RS language="rust" />
                            <FileRenderer
                                name="accordion.rs"
                                content=ACCORDION_RS
                                language="rust"
                            />
                        </Folder>
                        <FileRenderer
                            name="mod.rs"
                            content="Components module exports"
                            language="rust"
                        />
                    </Folder>
                    <Folder name="utils">
                        <FileRenderer name="hooks.rs" content=HOOKS_RS language="rust" />
                        <FileRenderer
                            name="mod.rs"
                            content="Utils module exports"
                            language="rust"
                        />
                    </Folder>
                </Folder>

                <FileRenderer
                    name="Cargo.toml"
                    content=CARGO_TOML
                    language="toml"
                />
                <FileRenderer
                    name="README.md"
                    content="Project documentation"
                    language="markdown"
                />
            </Tree>

            <Card class="flex-1">
                <CardContent>
                    <div id="content-display">
                        <div>
                            <h3 class="mb-2 font-semibold">main.rs</h3>
                            <pre class="overflow-x-auto p-4 text-sm rounded-md bg-muted syntax-highlighted">
                                <code inner_html=highlight_code(MAIN_RS, Some("rust"), Some("main.rs")) />
                            </pre>
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

const MAIN_RS: &str = r#"use leptos::prelude::*;
use rust_ui_labs::app::*;

fn main() {
    // Initialize logging
    console_error_panic_hook::set_once();

    // Mount the application
    mount_to_body(|| {
        view! { <App /> }
    });
}"#;

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


const CARGO_TOML: &str = r#"
[package]
name = "rust_ui_labs"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
icons = { version = "0.8", features = ["leptos", "leptos_animated"] }
leptos_ui = "0.2"
tw_merge = { version = "0.1", features = ["variant", "debug"] }


"#;