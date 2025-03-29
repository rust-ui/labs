use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::ui::button::{Button, ButtonVariant};

mod components {
    use super::*;
    clx! {DialogTitle, h3, "font-bold text-2xl"}
    clx! {DialogFooter, div, "flex justify-end gap-2"}
}
pub use components::*;

#[component]
pub fn DialogTrigger(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    children: Children,
) -> impl IntoView {
    view! {
        <Button onclick_string="modal.showModal()" class=class variant=variant>
            {children()}
        </Button>
    }
}

#[component]
pub fn Dialog(#[prop(into, optional)] class: Signal<String>, children: Children) -> impl IntoView {
    let class = Memo::new(move |_| {
        tw_merge!(
            "p-4 rounded-md w-full max-w-[600px]",
            "backdrop:bg-gray-400 backdrop:bg-opacity-50",
            // * Animations
            "open:animate-scale-in open:backdrop:animate-fade-in",
            class()
        )
    });

    const CLASS_BUTTON_CLOSE: &str = "absolute top-4 right-4";

    view! {
        <dialog class=class onclose="modalForm.reset()" id="modal" tabindex="-1">
            <button class=CLASS_BUTTON_CLOSE onclick="modal.close()">
                "X"
            </button>

            {children()}
        </dialog>
    }
}

#[component]
pub fn DialogClose(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    children: Children,
) -> impl IntoView {
    view! {
        <Button onclick_string="modal.close()" class=class variant=variant>
            {children()}
        </Button>
    }
}
