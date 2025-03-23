use dioxus::prelude::*;

const TAGS_ANIMATED_CSS: Asset = asset!("/assets/components/tags_animated.css");
const TAGS_ANIMATED_JS: Asset = asset!("/assets/components/tags_animated.js");

#[component]
pub fn DemoTagsAnimated() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAGS_ANIMATED_CSS }
        document::Script { src: TAGS_ANIMATED_JS }

        p { "Tags Animated" }

        div { class: "search" }

        // Tags section
        div { class: "tags",
            button {
                "Docker"
                span { "X" }
            }
            button {
                "Kubernetes"
                span { "X" }
            }
            button {
                "AWS"
                span { "X" }
            }
            button {
                "GraphQL"
                span { "X" }
            }
            button {
                "MongoDB"
                span { "X" }
            }
            button {
                "PostgreSQL"
                span { "X" }
            }
            button {
                "Redis"
                span { "X" }
            }
            button {
                "Git"
                span { "X" }
            }
            button {
                "Webpack"
                span { "X" }
            }
            button {
                "Vite"
                span { "X" }
            }
            button {
                "Cypress"
                span { "X" }
            }
            button {
                "Storybook"
                span { "X" }
            }
            button {
                "Tailwind"
                span { "X" }
            }
            button {
                "Prisma"
                span { "X" }
            }
            button {
                "Nginx"
                span { "X" }
            }
        }
    }
}
