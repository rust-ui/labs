use leptos::prelude::*;

use crate::shared::components::reactive_indicator::ReactiveIndicator;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex justify-between p-4">
            <div class="flex gap-4">
                <a href="/">"HOME"</a>
                <a href="/test/general">"Test 🧪"</a>
            </div>

            <div class="flex items-center gap-2">
                <ReactiveIndicator />
            // <ThemeToggle />
            </div>
        </div>
    }
}
