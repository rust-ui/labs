use leptos::prelude::*;

use super::{reactive_indicator::ReactiveIndicator, theme_toggle::ThemeToggle};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex justify-between p-4">
            <a href="/">"HOME 🧪"</a>

            <div class="flex items-center gap-4">
                <ReactiveIndicator />
                <ThemeToggle />
            </div>
        </div>
    }
}
