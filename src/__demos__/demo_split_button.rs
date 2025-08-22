use leptos::prelude::*;

#[component]
pub fn DemoSplitButton() -> impl IntoView {
    view! {
        <style>
            {"
            .split__button__container.open__split__button > .split__button__menu {
                display: block;
            }
            "}
        </style>

        <div class="box-border">
            <div class="flex p-20 mx-auto border border-border">
                <div class="inline-block relative mt-5 text-left split__button__container">
                    <button class="float-left relative px-3 text-sm border outline-none h-8 leading-7 bg-secondary text-secondary-foreground border-border shadow-sm hover:bg-accent active:bg-muted transition-colors duration-150">
                        "❖ Action"
                    </button>
                    <button class="float-left relative px-2 text-xs border border-l-0 outline-none h-8 leading-7 bg-secondary text-secondary-foreground border-border shadow-sm hover:bg-accent active:bg-muted transition-colors duration-150 split__button__dropdown">
                        "▼"
                    </button>
                    <ul class="hidden absolute right-0 top-8 z-50 min-w-40 text-sm list-none border bg-popover border-border shadow-md mt-0.5 py-1 split__button__menu">
                        <li>
                            <a
                                href="#"
                                class="block px-5 py-1.5 no-underline text-popover-foreground hover:bg-accent hover:text-accent-foreground transition-colors duration-200 split__button__menu__item"
                            >
                                Item - 1
                            </a>
                        </li>
                        <li>
                            <a
                                href="#"
                                class="block px-5 py-1.5 no-underline text-popover-foreground hover:bg-accent hover:text-accent-foreground transition-colors duration-200 split__button__menu__item"
                            >
                                Item - 2
                            </a>
                        </li>
                        <li>
                            <a
                                href="#"
                                class="block px-5 py-1.5 no-underline text-popover-foreground hover:bg-accent hover:text-accent-foreground transition-colors duration-200 split__button__menu__item"
                            >
                                Long Item - 3
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </div>

        <script>
            {"
            document.addEventListener('DOMContentLoaded', function() {
            const splitBtnContainer = document.querySelector('.split__button__container');
            const dropdownButton = document.querySelector('.split__button__dropdown');
            
            dropdownButton.addEventListener('click', function() {
            if (!splitBtnContainer.classList.contains('open__split__button')) {
             splitBtnContainer.classList.add('open__split__button');
            }
            });
            
            splitBtnContainer.addEventListener('click', function(event) {
            event.stopPropagation();
            });
            
            document.addEventListener('click', function() {
            if (splitBtnContainer.classList.contains('open__split__button')) {
             splitBtnContainer.classList.remove('open__split__button');
            }
            });
            });
            "}
        </script>
    }
}
