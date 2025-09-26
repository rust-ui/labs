use icons::{ChevronRight, Folder as FolderIcon, File as FileIcon};
use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Tree, div, "rounded-md border not-prose bg-neutral-50 w-[240px] border-neutral-200"}
    clx! {Folder, details, "flex flex-col group/tree-folder"}
    clx! {FileList, ul, "flex flex-col pl-2 ml-6"}
}

pub use components::*;


#[component]
pub fn DemoTreeView() -> impl IntoView {
    view! {
        <div class="p-10">
            <Tree>
                <Folder attr:open=true>
                    <FolderTrigger>
                        <span>"app"</span>
                    </FolderTrigger>
                    <FileList>
                        <File name="layout.tsx" group_name="file-selection" target_id="file-layout" checked=true />
                        <File name="page.tsx" group_name="file-selection" target_id="file-page" />
                        <File name="globals.css" group_name="file-selection" target_id="file-globals" />
                    </FileList>
                </Folder>
                <details
                    data-name="Folder"
                    data-state="Closed"
                    class="flex flex-col group/tree-folder"
                >
                    <summary
                        data-name="FolderTrigger"
                        class="flex flex-row gap-2 items-center py-1.5 px-2 w-full text-sm rounded-md cursor-pointer [>_svg]:size-4 hover:bg-accent hover:text-accent-foreground"
                    >
                        <svg
                            class="transition-transform duration-200 ease-in-out origin-center chevron group-open/tree-folder:rotate-90"
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                        <svg
                            class="size-4"
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <title>"Icon"</title>
                            <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"></path>
                        </svg>
                        <span>"components"</span>
                    </summary>
                    <ul data-name="FileList" class="flex flex-col pl-2 ml-6">
                        <li
                            data-name="File"
                            class="flex flex-row -ml-4 border-l border-neutral-300"
                        >
                            <input
                                id="file-button"
                                type="radio"
                                name="file-selection"
                                class="sr-only peer"
                            />
                            <label
                                for="file-button"
                                class="flex flex-row gap-2 items-center py-1.5 px-2 ml-3 w-full text-sm rounded-md cursor-pointer focus:outline-none [>_svg]:size-4 peer-checked:bg-neutral-200 peer-checked:font-medium hover:peer-checked:bg-neutral-200 hover:bg-accent hover:text-accent-foreground"
                                tabindex="0"
                            >
                                <FileIcon class="size-4" />
                                <span>"button.tsx"</span>
                            </label>
                        </li>
                        <li
                            data-name="File"
                            class="flex flex-row -ml-4 border-l border-neutral-300"
                        >
                            <input
                                id="file-input"
                                type="radio"
                                name="file-selection"
                                class="sr-only peer"
                            />
                            <label
                                for="file-input"
                                class="flex flex-row gap-2 items-center py-1.5 px-2 ml-3 w-full text-sm rounded-md cursor-pointer focus:outline-none [>_svg]:size-4 peer-checked:bg-neutral-200 peer-checked:font-medium hover:peer-checked:bg-neutral-200 hover:bg-accent hover:text-accent-foreground"
                                tabindex="0"
                            >
                                <FileIcon class="size-4" />
                                <span>"input.tsx"</span>
                            </label>
                        </li>
                    </ul>
                </details>
                <div data-name="File" class="flex flex-row">
                    <input
                        id="file-package"
                        type="radio"
                        name="file-selection"
                        class="sr-only peer"
                    />
                    <label
                        for="file-package"
                        class="flex flex-row gap-2 items-center py-1.5 px-2 w-full text-sm rounded-md cursor-pointer focus:outline-none [>_svg]:size-4 peer-checked:bg-neutral-200 peer-checked:font-medium hover:bg-accent hover:text-accent-foreground"
                        tabindex="0"
                    >
                        <FileIcon class="size-4" />
                        <span>"package.json"</span>
                    </label>
                </div>
            </Tree>
        </div>
    }
}



/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn FolderTrigger(children: Children) -> impl IntoView {
    view! {
        <summary
            data-name="FolderTrigger"
            class="flex flex-row gap-2 items-center py-1.5 px-2 w-full text-sm rounded-md cursor-pointer [&_svg:not([class*='size-'])]:size-4 hover:bg-accent hover:text-accent-foreground"
        >
            <ChevronRight class="transition-transform duration-200 ease-in-out origin-center group-open/tree-folder:rotate-90" />
            <FolderIcon />
            {children()}
        </summary>
    }
}

#[component]
pub fn File(
    #[prop(into)] name: &'static str,
    #[prop(into)] group_name: &'static str,
    #[prop(into)] target_id: &'static str,
    #[prop(default = false)] checked: bool,
) -> impl IntoView {
    view! {
        <li data-name="File" class="flex flex-row -ml-4 border-l border-neutral-300">
            <input id=target_id type="radio" name=group_name class="sr-only peer" checked=checked />
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

