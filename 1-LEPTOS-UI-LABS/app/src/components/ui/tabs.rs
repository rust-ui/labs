use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::utils::random_id::generate_random_id;

mod components {
    use super::*;
    clx! {Tabs, div, "flex flex-wrap"}
    clx! {TabsContent, div, "tabs__item-content", "order-last", "flex-grow hidden w-full p-2 border rounded-lg border-input"}
}

pub use components::*;

#[component]
pub fn TabsTrigger(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] checked: Option<bool>,
    #[prop(into, optional)] name: Option<String>,
    children: Children,
) -> impl IntoView {
    let random_id = generate_random_id();

    let class_label = Memo::new(move |_| {
        tw_merge!(
            "tabs__item-label",
            "order-1",
            "font-bold cursor-pointer block p-4 mr-[0.2rem] border-b-[0.2rem] border-transparent",
            class()
        )
    });

    view! {
        <input
            class="hidden tabs__item-input"
            type="radio"
            name=name.unwrap_or("tabs".to_string())
            id=random_id.clone()
            checked=checked
        />
        <label class=class_label for=random_id>
            {children()}
        </label>
    }
}
