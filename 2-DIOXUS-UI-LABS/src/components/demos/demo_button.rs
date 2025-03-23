use dioxus::prelude::*;

use crate::components::ui::button::Button;

#[component]
pub fn DemoButton() -> Element {
    rsx! {
        div { class: "p-4 flex flex-col gap-4",
            h1 { "Test Page" }

            Button { "Primary" }
            Button { class: "bg-sky-500", "Custom class" }
        }
    }
}
