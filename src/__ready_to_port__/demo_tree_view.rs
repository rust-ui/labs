use icons::{ChevronRight, Folder as FolderIcon, File as FileIcon};
use leptos::prelude::*;
use leptos_ui::clx;
use crate::utils::hooks::use_random::use_random_id;

mod components {
    use super::*;
    clx! {Tree, div, "rounded-md border not-prose bg-card w-[240px] border-border"}
    clx! {FolderTrigger, label, "flex flex-row gap-2 items-center py-1.5 px-2 w-full text-sm rounded-md cursor-pointer [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground"}
    clx! {FolderContent, div, "grid overflow-hidden transition-all duration-400 grid-rows-[0fr] peer-checked:grid-rows-[1fr]"}
    clx! {FileList, ul, "flex flex-col pl-2 ml-6 relative before:content-[''] before:absolute before:-left-2 before:top-0 before:bottom-0 before:border-l before:border-muted-foreground/30 min-h-[0]"}
}

pub use components::*;


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



/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */


#[component]
pub fn Folder(
    #[prop(into)] name: &'static str,
    #[prop(default = false)] open: bool,
    children: Children,
) -> impl IntoView {
    let folder_id = use_random_id();

    view! {
        <div
            data-name="Folder"
            class="flex flex-col [&:has(>input:checked)>label>svg:first-child]:rotate-90"
        >
            <input id=folder_id.clone() type="checkbox" class="sr-only peer" checked=open />

            <label
                for=folder_id
                class="flex flex-row gap-2 items-center py-1.5 px-2 w-full text-sm rounded-md cursor-pointer [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground"
            >
                <ChevronRight class="transition-transform duration-200 ease-in-out origin-center" />
                <FolderIcon />
                <span>{name}</span>
            </label>

            <FolderContent>
                <FileList>{children()}</FileList>
            </FolderContent>
        </div>
    }
}

#[component]
pub fn File(
    #[prop(into)] name: &'static str,
    #[prop(default = false)] checked: bool,
) -> impl IntoView {
    let target_id = use_random_id();

    view! {
        <li data-name="File" class="flex flex-row -ml-4">
            <input
                id=target_id.clone()
                type="radio"
                name="file-selection"
                class="sr-only peer"
                checked=checked
            />
            <label
                for=target_id
                class="flex flex-row gap-2 items-center py-1.5 px-2 ml-3 w-full text-sm rounded-md cursor-pointer focus:outline-none [>_svg]:size-4 peer-checked:bg-accent peer-checked:font-medium hover:peer-checked:bg-accent hover:bg-accent hover:text-accent-foreground"
                tabindex="0"
            >
                <FileIcon class="size-4" />
                <span>{name}</span>
            </label>
        </li>
    }
}

