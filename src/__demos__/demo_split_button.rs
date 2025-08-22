use icons::{ChevronDown, Copy};
use leptos::prelude::*;
use leptos_ui::{a, clx};

mod components {
    use super::*;
    clx! {SplitButtonContainer, div, "inline-block relative mt-5 text-left", "split__button__container"}
    clx! {MyButton, button, "float-left relative px-4 h-9 text-sm font-medium border rounded-l-md shadow-sm transition-colors duration-150 outline-none bg-white text-gray-700 border-gray-300 hover:bg-gray-50 active:bg-gray-100"}
    clx! {DropdownButton, button, "float-left relative px-2 h-9 text-xs border border-l-0 rounded-r-md shadow-sm transition-colors duration-150 outline-none bg-white text-gray-700 border-gray-300", "split__button__dropdown", "hover:bg-gray-50 active:bg-gray-100"}
    clx! {DropdownMenu, ul, "hidden absolute right-0 top-9 z-50 py-1 mt-1 text-sm list-none border rounded-md shadow-lg min-w-40 bg-white border-gray-200", "split__button__menu"}
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
            .split__button__container.open__split__button > .split__button__menu {
            display: block;
            }
            "}
        </style>

        <div class="flex p-20 mx-auto border border-border">
            <SplitButtonContainer>
                <MyButton>
                    <Copy class="mr-2 size-4" />
                    "Copy Page"
                </MyButton>
                <DropdownButton>
                    <ChevronDown class="size-4" />
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
            </SplitButtonContainer>
        </div>

        <script>
            // **IMPORTANT**: DO NOT MODIFY.
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
