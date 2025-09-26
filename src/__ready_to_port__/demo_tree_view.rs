use leptos::prelude::*;

use crate::components::ui::tree_view::{Tree, File, Folder};


#[component]
pub fn DemoTreeView() -> impl IntoView {
    view! {
        <div class="flex flex-col items-start justify-start h-full mt-10">
            <Tree>
            <Folder name="src" open=true>
                <File name="main.rs" checked=true />
                <File name="lib.rs" />
                <Folder name="components" open=true>
                    <Folder name="ui">
                        <File name="button.rs" />
                        <File name="card.rs" />
                        <File name="input.rs" />
                        <File name="accordion.rs" />
                    </Folder>
                    <File name="mod.rs" />
                </Folder>
                <Folder name="utils">
                    <File name="hooks.rs" />
                    <File name="mod.rs" />
                </Folder>
            </Folder>
            </Tree>
        </div>
    }
}

