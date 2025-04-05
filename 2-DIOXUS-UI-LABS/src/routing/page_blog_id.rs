use dioxus::prelude::*;

use crate::routes::Routes;

#[component]
pub fn PageBlogId(id: i32) -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { class: "text-green-500", "This is blog #{id}!" }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            // Navigation links
            Link { to: Routes::PageBlogId { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Routes::PageBlogId { id: id + 1 }, "Next" }
        }
    }
}
