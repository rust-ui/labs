use leptos::prelude::*;

#[component]
pub fn DemoTreeView() -> impl IntoView {
    view! {
        <div class="p-10">
            <div data-name="Tree" class="not-prose bg-neutral-50 w-[240px] rounded-md border border-neutral-200">
                <details data-name="Folder" data-state="Open" class="flex flex-col group/tree-folder" open=true>
                    <summary data-name="FolderContent" class="flex flex-row items-center gap-2 rounded-md px-2 py-1.5 text-sm hover:bg-accent hover:text-accent-foreground [>_svg]:size-4 w-full cursor-pointer">
                        <svg class="chevron transition-transform duration-200 ease-in-out origin-center group-open/tree-folder:rotate-90" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <title>"Icon"</title>
                            <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"></path>
                        </svg>
                        <span>"app"</span>
                    </summary>
                    <ul data-name="FileList" class="flex flex-col pl-2 ml-6">
                        <li data-name="File" class="flex flex-row border-l border-neutral-300 -ml-4">
                            <input id="file-layout" type="radio" name="file-selection" class="sr-only peer" checked=true />
                            <label for="file-layout" class="ml-3 flex flex-row items-center gap-2 rounded-md px-2 py-1.5 text-sm hover:bg-accent hover:text-accent-foreground [>_svg]:size-4 cursor-pointer focus:outline-none w-full peer-checked:bg-neutral-200 peer-checked:font-medium" tabindex="0">
                                <svg class="size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <title>"Icon"</title>
                                    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
                                    <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
                                </svg>
                                <span>"layout.tsx"</span>
                            </label>
                        </li>
                        <li data-name="File" class="flex flex-row border-l border-neutral-300 -ml-4">
                            <input id="file-page" type="radio" name="file-selection" class="sr-only peer" />
                            <label for="file-page" class="ml-3 flex flex-row items-center gap-2 rounded-md px-2 py-1.5 text-sm hover:bg-accent hover:text-accent-foreground [>_svg]:size-4 cursor-pointer focus:outline-none w-full peer-checked:bg-neutral-200 peer-checked:font-medium" tabindex="0">
                                <svg class="size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <title>"Icon"</title>
                                    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
                                    <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
                                </svg>
                                <span>"page.tsx"</span>
                            </label>
                        </li>
                        <li data-name="File" class="flex flex-row border-l border-neutral-300 -ml-4">
                            <input id="file-global" type="radio" name="file-selection" class="sr-only peer" />
                            <label for="file-global" class="ml-3 flex flex-row items-center gap-2 rounded-md px-2 py-1.5 text-sm hover:bg-accent hover:text-accent-foreground [>_svg]:size-4 cursor-pointer focus:outline-none w-full peer-checked:bg-neutral-200 peer-checked:font-medium" tabindex="0">
                                <svg class="size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <title>"Icon"</title>
                                    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
                                    <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
                                </svg>
                                <span>"global.css"</span>
                            </label>
                        </li>
                    </ul>
                </details>
                <details data-name="Folder" data-state="Closed" class="flex flex-col group/tree-folder">
                    <summary data-name="FolderContent" class="flex flex-row items-center gap-2 rounded-md px-2 py-1.5 text-sm hover:bg-accent hover:text-accent-foreground [>_svg]:size-4 w-full cursor-pointer">
                        <svg class="chevron transition-transform duration-200 ease-in-out origin-center group-open/tree-folder:rotate-90" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M9 18l6-6-6-6"></path>
                        </svg>
                        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <title>"Icon"</title>
                            <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"></path>
                        </svg>
                        <span>"components"</span>
                    </summary>
                    <ul data-name="FileList" class="flex flex-col pl-2 ml-6">
                        <li data-name="File" class="flex flex-row border-l border-neutral-300 -ml-4">
                            <input id="file-button" type="radio" name="file-selection" class="sr-only peer" />
                            <label for="file-button" class="ml-3 flex flex-row items-center gap-2 rounded-md px-2 py-1.5 text-sm hover:bg-accent hover:text-accent-foreground [>_svg]:size-4 cursor-pointer focus:outline-none w-full peer-checked:bg-neutral-200 peer-checked:font-medium" tabindex="0">
                                <svg class="size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <title>"Icon"</title>
                                    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
                                    <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
                                </svg>
                                <span>"button.tsx"</span>
                            </label>
                        </li>
                        <li data-name="File" class="flex flex-row border-l border-neutral-300 -ml-4">
                            <input id="file-input" type="radio" name="file-selection" class="sr-only peer" />
                            <label for="file-input" class="ml-3 flex flex-row items-center gap-2 rounded-md px-2 py-1.5 text-sm hover:bg-accent hover:text-accent-foreground [>_svg]:size-4 cursor-pointer focus:outline-none w-full peer-checked:bg-neutral-200 peer-checked:font-medium" tabindex="0">
                                <svg class="size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <title>"Icon"</title>
                                    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
                                    <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
                                </svg>
                                <span>"input.tsx"</span>
                            </label>
                        </li>
                    </ul>
                </details>
                <div data-name="File" class="flex flex-row">
                    <input id="file-package" type="radio" name="file-selection" class="sr-only peer" />
                    <label for="file-package" class="flex flex-row items-center gap-2 rounded-md px-2 py-1.5 text-sm hover:bg-accent hover:text-accent-foreground [>_svg]:size-4 cursor-pointer focus:outline-none w-full peer-checked:bg-neutral-200 peer-checked:font-medium" tabindex="0">
                        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <title>"Icon"</title>
                            <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
                            <path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
                        </svg>
                        <span>"package.json"</span>
                    </label>
                </div>
            </div>
        </div>
    }
}