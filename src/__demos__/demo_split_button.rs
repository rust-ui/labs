use icons::{ChevronDown, Copy};
use leptos::prelude::*;
use leptos_ui::{a, clx};

mod components {
    use super::*;
    clx! {SplitButton, div, "split__button", "inline-flex relative"}
    clx! {Button, button, "flex items-center px-3 h-8 text-sm font-medium border rounded-l-md transition-colors duration-150 outline-none bg-card text-card-foreground border-border hover:bg-accent hover:text-accent-foreground active:bg-muted [&_svg:not(:last-child)]:mr-2 [&_svg:not(:first-child)]:ml-2"}
    clx! {DropdownButton, button, "split__button__dropdown", "flex items-center px-1.5 h-8 border border-l-0 rounded-r-md transition-colors duration-150 outline-none bg-card text-card-foreground border-border hover:bg-accent hover:text-accent-foreground active:bg-muted"}
    clx! {DropdownMenu, ul, "split__button__menu", "hidden absolute right-0 top-8 z-50 py-1 mt-1 text-sm list-none border rounded-md shadow-lg min-w-40 bg-popover border-border"}
    clx! {MenuItem, li, ""}
    a! {MenuLink, "split__button__menu__item", "block py-2 px-4 no-underline transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground"}
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
                    <MenuLink href="#">Item - 3</MenuLink>
                </MenuItem>
            </DropdownMenu>
        </SplitButton>

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
