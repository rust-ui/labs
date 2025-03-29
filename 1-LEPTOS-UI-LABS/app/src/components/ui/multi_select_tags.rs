use leptos::prelude::*;
use leptos_ui::{clx, div};

mod components {
    use super::*;
    div! {MultiSelectTagsSearch, "search"}
    clx! {MultiSelectTags, div, "tags"}
}

pub use components::*;
