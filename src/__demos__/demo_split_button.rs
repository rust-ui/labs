use icons::ChevronDown;
use leptos::prelude::*;
use leptos_ui::{a, clx};

mod components {
    use super::*;
    clx! {SplitButtonContainer, div, "inline-block relative mt-5 text-left", "split__button__container"}
    clx! {MyButton, button, "float-left relative px-3 h-8 text-sm leading-7 border shadow-sm transition-colors duration-150 outline-none bg-secondary text-secondary-foreground border-border hover:bg-accent active:bg-muted"}
    clx! {DropdownButton, button, "float-left relative px-2 h-8 text-xs leading-7 border border-l-0 shadow-sm transition-colors duration-150 outline-none bg-secondary text-secondary-foreground border-border", "split__button__dropdown", "hover:bg-accent active:bg-muted"}
    clx! {DropdownMenu, ul, "hidden absolute right-0 top-8 z-50 py-1 mt-0.5 text-sm list-none border shadow-md min-w-40 bg-popover border-border", "split__button__menu"}
    clx! {MenuItem, li, ""}
    a! {MenuLink, "block py-1.5 px-5 no-underline transition-colors duration-200 text-popover-foreground", "split__button__menu__item", "hover:bg-accent hover:text-accent-foreground"}
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
                <MyButton>"My Button"</MyButton>
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
