use leptos::prelude::*;
use tw_merge::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn DialogTrigger(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| tw_merge!("", class()));

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
pub fn DialogTitle(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| tw_merge!("font-bold text-2xl", class()));

    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn DialogFooter(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| tw_merge!("flex justify-end gap-2", class()));

    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn DialogClose(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| tw_merge!("", class()));

    view! {
        <Button onclick_string="modal.close()" class=class variant=variant>
            {children()}
        </Button>
    }
}
