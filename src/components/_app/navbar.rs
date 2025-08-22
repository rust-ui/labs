use leptos::prelude::*;

use super::reactive_indicator::ReactiveIndicator;
use super::theme_toggle::ThemeToggle;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex sticky top-0 z-50 justify-between p-4 bg-background">
            <div class="flex gap-2">
                <a href="/">"HOME 🧪"</a>
                // Need full reload of the page for the animation to load properly
                <a href="#" onclick="window.location.href='/gsap-intro'; return false;">
                    "Gsap intro"
                </a>
            </div>

            <div class="flex gap-4 items-center">
                <ReactiveIndicator />
                <ThemeToggle />
            </div>
        </div>
    }
}
