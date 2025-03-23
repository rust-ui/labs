use dioxus::prelude::*;

use crate::{
    routes::Routes,
    shared::constants::{FAVICON, TAILWIND_CSS},
};

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Routes> {}
    }
}
