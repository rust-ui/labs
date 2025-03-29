use leptos::prelude::*;
use leptos_ui::{clx, div};

mod components {
    use super::*;
    div! {MultiSelectTagsSearch, "search flex flex-wrap gap-3 w-[400px] min-h-[48px] p-3 border border-gray-300 rounded-lg text-base"}
    clx! {MultiSelectTags, div, "tags flex flex-wrap gap-3 p-3 bg-white rounded-2xl border border-gray-300 w-[400px]"}
}

pub use components::*;
