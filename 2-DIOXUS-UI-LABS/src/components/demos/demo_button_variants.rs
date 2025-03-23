use dioxus::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn DemoButtonVariants() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            Button { "Default Button" }
            Button { variant: ButtonVariant::Secondary, "Secondary Button" }
            Button { variant: ButtonVariant::Destructive, "Destructive Button" }
            Button { variant: ButtonVariant::Warning, size: ButtonSize::Sm, "Small Warning Button" }
            Button { variant: ButtonVariant::Success, size: ButtonSize::Lg, "Large Success Button" }
            Button { variant: ButtonVariant::Success, size: ButtonSize::Icon, "👍" }
        }
    }
}
