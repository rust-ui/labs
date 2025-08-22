use leptos::prelude::*;
use leptos_meta::Stylesheet;

use crate::components::ui::multi_select_tags::{MultiSelectTags, MultiSelectTagsSearch, TagItem};

#[component]
pub fn DemoJsMultiSelectTags() -> impl IntoView {
    view! {
        <Stylesheet id="multi_select_tags" href="/components/multi_select_tags.css" />

        <MultiSelectTagsSearch />

        <MultiSelectTags>
            {TAGS.iter().map(|&tag| view! { <TagItem>{tag}</TagItem> }).collect::<Vec<_>>()}
        </MultiSelectTags>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const TAGS: [&str; 15] = [
    "Docker",
    "Kubernetes",
    "AWS",
    "GraphQL",
    "MongoDB",
    "PostgreSQL",
    "Redis",
    "Git",
    "Webpack",
    "Vite",
    "Cypress",
    "Storybook",
    "Tailwind",
    "Prisma",
    "Nginx",
];
