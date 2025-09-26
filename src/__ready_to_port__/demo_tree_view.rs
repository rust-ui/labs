use icons::{ChevronRight, Folder as FolderIcon, File as FileIcon};
use leptos::prelude::*;
use leptos_ui::clx;
use crate::utils::hooks::use_random::use_random_id;

mod components {
    use super::*;
    clx! {Tree, div, "rounded-md border not-prose bg-neutral-50 w-[240px] border-neutral-200"}
}

pub use components::*;


#[component]
pub fn DemoTreeView() -> impl IntoView {
    view! {
            <Tree>
                <Folder name="app" open=true>
                    <File name="layout.tsx" checked=true />
                    <File name="page.tsx" />
                    <File name="globals.css" />
                    <Folder name="dashboard">
                        <File name="page.tsx" />
                        <File name="layout.tsx" />
                    </Folder>
                </Folder>

                <Folder name="components">
                    <File name="button.tsx" />
                    <File name="input.tsx" />
                </Folder>

                 <File name="package.json" />
            </Tree>
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
    view! {
        <details data-name="Folder" class="flex flex-col open:[&>summary>svg:first-child]:rotate-90" prop:open=open>
            <summary
                data-name="FolderTrigger"
                class="flex flex-row gap-2 items-center py-1.5 px-2 w-full text-sm rounded-md cursor-pointer [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground"
            >
                <ChevronRight class="transition-transform duration-200 ease-in-out origin-center" />
                <FolderIcon />
                <span>{name}</span>
            </summary>

            <ul data-name="FileList" class="flex flex-col pl-2 ml-6 relative before:content-[''] before:absolute before:-left-2 before:top-0 before:bottom-0 before:border-l before:border-neutral-300">
                {children()}
            </ul>
        </details>
    }
}

#[component]
pub fn File(
    #[prop(into)] name: &'static str,
    #[prop(default = false)] checked: bool,
) -> impl IntoView {
    let target_id = use_random_id();

    view! {
        <li data-name="File" class="flex flex-row [details_&]:-ml-4">
            <input id=target_id.clone() type="radio" name="file-selection" class="sr-only peer" checked=checked />
            <label
                for=target_id
                class="flex flex-row gap-2 items-center py-1.5 px-2 ml-3 w-full text-sm rounded-md cursor-pointer focus:outline-none [>_svg]:size-4 peer-checked:bg-neutral-200 peer-checked:font-medium hover:peer-checked:bg-neutral-200 hover:bg-accent hover:text-accent-foreground"
                tabindex="0"
            >
                <FileIcon class="size-4" />
                <span>{name}</span>
            </label>
        </li>
    }
}

