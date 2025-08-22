use icons::{ChevronDown, Copy};
use leptos::prelude::*;
use leptos_ui::{a, clx};

mod components {
    use super::*;
    clx! {SplitButton, div, "inline-flex relative", "split__button"}
    clx! {Button, button, "flex items-center px-3 h-8 text-sm font-medium border rounded-l-md transition-colors duration-150 outline-none bg-white text-gray-700 border-gray-300 hover:bg-gray-50 active:bg-gray-100 [&_svg:not(:last-child)]:mr-2 [&_svg:not(:first-child)]:ml-2"}
    clx! {DropdownButton, button, "flex items-center px-1.5 h-8 border border-l-0 rounded-r-md transition-colors duration-150 outline-none bg-white text-gray-700 border-gray-300", "split__button__dropdown", "hover:bg-gray-50 active:bg-gray-100"}
    clx! {DropdownMenu, ul, "hidden absolute right-0 top-8 z-50 py-1 mt-1 text-sm list-none border rounded-md shadow-lg min-w-40 bg-white border-gray-200", "split__button__menu"}
    clx! {MenuItem, li, ""}
    a! {MenuLink, "block py-2 px-4 no-underline transition-colors duration-200 text-gray-700", "split__button__menu__item", "hover:bg-gray-100 hover:text-gray-900"}
}

pub use components::*;

#[component]
pub fn DemoSplitButton() -> impl IntoView {
    view! {
        <style>
            // **IMPORTANT**: DO NOT MODIFY.
            {"
            .split__button.open__split__button > .split__button__menu {
            display: block;
            }
            "}
        </style>

        <div class="flex p-20 mx-auto border border-border">
            <SplitButton>
                <Button>
                    <Copy class="size-3" />
                    <span>"Copy Page"</span>
                </Button>

                <DropdownButton>
                    <ChevronDown class="size-3" />
                </DropdownButton>
                <DropdownMenu>
                    <MenuItem>
                        <MenuLink href="#">Item - 1</MenuLink>
                    </MenuItem>
                    <MenuItem>
                        <MenuLink href="#">Item - 2</MenuLink>
                    </MenuItem>
                    <MenuItem>
                        <MenuLink href="#">Long Item - 3</MenuLink>
                    </MenuItem>
                </DropdownMenu>
            </SplitButton>
        </div>

        <script>
            // **IMPORTANT**: DO NOT MODIFY.
            {"
            document.addEventListener('DOMContentLoaded', function() {
            const splitButton = document.querySelector('.split__button');
            const dropdownButton = document.querySelector('.split__button__dropdown');
            
            dropdownButton.addEventListener('click', function() {
            if (!splitButton.classList.contains('open__split__button')) {
            splitButton.classList.add('open__split__button');
            }
            });
            
            splitButton.addEventListener('click', function(event) {
            event.stopPropagation();
            });
            
            document.addEventListener('click', function() {
            if (splitButton.classList.contains('open__split__button')) {
            splitButton.classList.remove('open__split__button');
            }
            });
            });
            "}
        </script>
    }
}
