use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Container, div, "container bg-[#333] p-4 mt-4"}
}

pub use components::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn DraggableItem(text: String) -> impl IntoView {
    view! {
        <p class="p-4 bg-white border border-black cursor-move draggable" draggable="true">{text}</p>
    }
}
#[component]
pub fn DemoDragAndDrop() -> impl IntoView {
    view! {
        <Stylesheet href="/components/drag_and_drop.css" />

        <div class="relative gap-8">
            <div class="m-0">
                <Container>
                    <DraggableItem text="1".into() />
                    <DraggableItem text="2".into() />
                </Container>
                <Container>
                    <DraggableItem text="3".into() />
                    <DraggableItem text="4".into() />
                </Container>
            </div>
        </div>
        <script src="/components/drag_and_drop.js"></script>
    }
}
