use dioxus::prelude::*;

use crate::components::ui::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};

#[component]
pub fn DemoCard() -> Element {
    rsx! {
        div { class: "p-4 flex flex-col gap-4",
            Card { class: "w-full max-w-[500px]",
                CardHeader {
                    CardTitle { class: "text-red-500", "Card Title" }
                }
                CardContent { "Hello" }
                CardFooter { "footer" }
            }
        }
    }
}
