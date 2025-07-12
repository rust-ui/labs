use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::app::App;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        // <html lang="en" class="dark">
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />

                // <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>

                <script src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.12.2/gsap.min.js"></script>
                <script src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.12.2/ScrollTrigger.min.js"></script>

                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>

            <body>
                <App />
            </body>
        </html>
    }
}
