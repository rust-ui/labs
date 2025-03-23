use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoJsTagsAnimated() -> impl IntoView {
    view! {
        <Stylesheet id="tags-animated" href="/components/tags_animated.css" />
        <script src="/components/tags_animated.js" />

        <h1>TAGS</h1>
        <div class="search"></div>

        <div class="tags">
            <button>Docker<span>X</span></button>
            <button>Kubernetes<span>X</span></button>
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
        </div>
    }
}
