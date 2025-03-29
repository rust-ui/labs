use leptos::prelude::*;
use leptos_ui::{clx, div};

mod components {
    use super::*;
    div! {MultiSelectTagsSearch, "search flex flex-wrap gap-3 w-[400px] min-h-[48px] p-3 border border-gray-300 rounded-lg text-base"}
    clx! {MultiSelectTags, div, "tags flex flex-wrap gap-3 p-3 bg-white rounded-2xl border border-gray-300 w-[400px]"}
    clx! {RootItem, button, "p-2 bg-gray-200 rounded-lg text-base text-gray-800 cursor-pointer border-none font-medium flex items-center gap-2"}
}

pub use components::*;

#[component]
pub fn TagItem(children: Children) -> impl IntoView {
    view! { <RootItem>{children()} <span class="display-none">X</span></RootItem> }
}
