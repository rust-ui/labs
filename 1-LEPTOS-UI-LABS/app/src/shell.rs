use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::app::App;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <script src="https://cdn.tailwindcss.com"></script>

                // ------- DEMO CHART JS -------
                <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>

                <script>
                    "tailwind.config = {
                        darkMode: 'class'
                    }"
                </script>
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
