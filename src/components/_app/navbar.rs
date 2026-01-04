use leptos::prelude::*;

use super::reactive_indicator::ReactiveIndicator;
use super::theme_toggle::ThemeToggle;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex sticky top-0 z-50 justify-between p-4 pt-[calc(1rem+env(safe-area-inset-top))] bg-background">
            <div class="flex gap-2 items-center">
                <a href="/" class="font-semibold">
                    "Rust/UI Labs"
                </a>
                // Hide nav links on mobile (bottom nav handles it)
                <nav class="hidden gap-2 ml-4 md:flex">
                    <a href="/test" class="text-muted-foreground hover:text-foreground">
                        "Test"
                    </a>
                    <a
                        href="#"
                        onclick="window.location.href='/gsap-intro'; return false;"
                        class="text-muted-foreground hover:text-foreground"
                    >
                        "Gsap intro"
                    </a>
                </nav>
            </div>

            <div class="flex gap-4 items-center">
                <ReactiveIndicator />
                <ThemeToggle />
            </div>
        </div>
    }
}
