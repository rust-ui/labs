use leptos::prelude::*;
use leptos_ui::{clx, div};

use crate::utils::utils::Utils;

mod components {
    use super::*;
    div! {MultiSelectTagsSearch, "search flex flex-wrap gap-3 w-[400px] min-h-[48px] p-3 border border-input rounded-lg"}
    clx! {TagsRoot, div, "tags flex flex-wrap gap-3 p-3 rounded-2xl border border-input w-[400px]"}
    clx! {RootItem, button, "flex items-center gap-2 p-2 rounded-lg cursor-pointer text-muted-foreground border-none"}
}

pub use components::*;

#[component]
pub fn TagItem(children: Children) -> impl IntoView {
    let random_id = Utils::use_random_id();

    let transition_name = format!(
        "view-transition-name: tag-{}; order: {}",
        random_id, random_id
    );

    view! {
        <RootItem
            onclick="const tag = event.target.closest('button'); if (tag) { document.startViewTransition(() => { document.querySelector('.search').appendChild(tag); }); }"
            style=transition_name
        >
            {children()}
            <span class="display-none">X</span>
        </RootItem>
    }
}

#[component]
pub fn MultiSelectTags(children: Children) -> impl IntoView {
    view! { <TagsRoot>{children()}</TagsRoot> }
}
