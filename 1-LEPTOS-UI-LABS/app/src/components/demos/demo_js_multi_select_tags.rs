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
            <TagItem>Docker</TagItem>
            <TagItem>Kubernetes</TagItem>
            <TagItem>AWS</TagItem>
            <TagItem>GraphQL</TagItem>
            <TagItem>MongoDB</TagItem>
            <TagItem>PostgreSQL</TagItem>
            <TagItem>Redis</TagItem>
            <TagItem>Git</TagItem>
            <TagItem>Webpack</TagItem>
            <TagItem>Vite</TagItem>
            <TagItem>Cypress</TagItem>
            <TagItem>Storybook</TagItem>
            <TagItem>Tailwind</TagItem>
            <TagItem>Prisma</TagItem>
            <TagItem>Nginx</TagItem>
        </MultiSelectTags>
    }
}
