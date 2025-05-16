use leptos::prelude::*;
use leptos_ui::utils::Utils;
use leptos_ui::{clx, transition};

#[component]
pub fn DemoAnimatedCardRemoval() -> impl IntoView {
    view! {
        <div class="flex justify-center w-full gap-8 max-w-[1000px]">
           <CardItemDemo />
           <CardItemDemo />
           <CardItemDemo />
           <CardItemDemo />
           <CardItemDemo />
        </div>

        <script src="/components/cards_removal.js"></script>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
fn CardItemDemo() -> impl IntoView {
    view! {
        <CardItem>
            <DeleteButtonWrapper class="delete-btn">
                <span>"🗑️"</span>
            </DeleteButtonWrapper>
        </CardItem>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

mod components {
    use super::*;

    transition! {CardItem, li, "w-full aspect-[2/3] block relative rounded-xl max-w-[220px] bg-neutral-400"}
    clx! {DeleteButtonWrapper, button, "cursor-pointer absolute -bottom-3 -right-3 size-12 p-2 border-4 rounded-full bg-neutral-100"}
}

pub use components::*;
