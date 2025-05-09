use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Draggable, div, "dragabble__container", "bg-[#333] p-4 mt-4"}
}

pub use components::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn DraggableItem(#[prop(into)] text: String) -> impl IntoView {
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
                <Draggable>
                    <DraggableItem text="1" />
                    <DraggableItem text="2" />
                </Draggable>
                <Draggable>
                    <DraggableItem text="3" />
                    <DraggableItem text="4" />
                </Draggable>
            </div>
        </div>
        <script src="/components/drag_and_drop.js"></script>
    }
}
