use leptos::prelude::*;

use super::{reactive_indicator::ReactiveIndicator, theme_toggle::ThemeToggle};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex justify-between p-4 z-50 sticky top-0 bg-background">
            <div class="flex gap-2">
                <a href="/">"HOME 🧪"</a>
                // Need full reload of the page for the animation to load properly
                <a href="#" onclick="window.location.href='/gsap-intro'; return false;">
                    "Gsap intro"
                </a>
            </div>

            <div class="flex items-center gap-4">
                <ReactiveIndicator />
                <ThemeToggle />
            </div>
        </div>
    }
}
