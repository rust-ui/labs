use leptos::prelude::*;
use leptos_meta::Stylesheet;

use crate::components::ui::multi_select_tags::{MultiSelectTags, MultiSelectTagsSearch};

#[component]
pub fn DemoJsMultiSelectTags() -> impl IntoView {
    view! {
        <Stylesheet id="tags-animated" href="/components/tags_animated.css" />
        <script src="/components/tags_animated.js" />

        <MultiSelectTagsSearch />

        <MultiSelectTags>
            // <TagItem>Docker</TagItem>
            // <TagItem>Kubernetes</TagItem>
            <button>AWS<span>X</span></button>
            <button>GraphQL<span>X</span></button>
            <button>MongoDB<span>X</span></button>
            <button>PostgreSQL<span>X</span></button>
            <button>Redis<span>X</span></button>
            <button>Git<span>X</span></button>
            <button>Webpack<span>X</span></button>
            <button>Vite<span>X</span></button>
            <button>Cypress<span>X</span></button>
            <button>Storybook<span>X</span></button>
            <button>Tailwind<span>X</span></button>
            <button>Prisma<span>X</span></button>
            <button>Nginx<span>X</span></button>
        </MultiSelectTags>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn TagItem(children: Children) -> impl IntoView {
    view! { <button>{children()} <span>"X"</span></button> }
}
