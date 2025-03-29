use leptos::prelude::*;
use leptos_ui::{clx, div};

use crate::utils::utils::Utils;

mod components {
    use super::*;
    div! {MultiSelectTagsSearch, "search flex flex-wrap gap-3 w-[400px] min-h-[48px] p-3 border border-input rounded-lg"}
    clx! {MultiSelectTags, div, "tags flex flex-wrap gap-3 p-3 rounded-2xl border border-input w-[400px]"}
    clx! {RootItem, button, "flex items-center gap-2 p-2 rounded-lg cursor-pointer text-muted-foreground border-none"}
}

pub use components::*;

#[component]
pub fn TagItem(children: Children) -> impl IntoView {
    let random_name = Utils::use_random_transition_name();

    view! { <RootItem>{children()} <span class="display-none">X</span></RootItem> }
}
