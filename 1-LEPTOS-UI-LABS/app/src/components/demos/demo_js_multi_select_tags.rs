use leptos::prelude::*;
use leptos_meta::Stylesheet;

use crate::components::ui::multi_select_tags::{MultiSelectTags, MultiSelectTagsSearch, TagItem};

#[component]
pub fn DemoJsMultiSelectTags() -> impl IntoView {
    view! {
        <Stylesheet id="tags-animated" href="/components/tags_animated.css" />
        <script src="/components/tags_animated.js" />

        <MultiSelectTagsSearch />

        <MultiSelectTags>
            // <TagItem>Docker</TagItem>
            // <TagItem>Kubernetes</TagItem>
            <TagItem>AWS<span>X</span></TagItem>
            <TagItem>GraphQL<span>X</span></TagItem>
            <TagItem>MongoDB<span>X</span></TagItem>
            <TagItem>PostgreSQL<span>X</span></TagItem>
            <TagItem>Redis<span>X</span></TagItem>
            <TagItem>Git<span>X</span></TagItem>
            <TagItem>Webpack<span>X</span></TagItem>
            <TagItem>Vite<span>X</span></TagItem>
            <TagItem>Cypress<span>X</span></TagItem>
            <TagItem>Storybook<span>X</span></TagItem>
            <TagItem>Tailwind<span>X</span></TagItem>
            <TagItem>Prisma<span>X</span></TagItem>
            <TagItem>Nginx<span>X</span></TagItem>
        </MultiSelectTags>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/
