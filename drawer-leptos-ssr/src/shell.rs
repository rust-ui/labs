use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::app::App;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        // <html lang="en" class="dark">
        <html lang="en" data-snap="true" data-meta="true" data-content="true">
            <head>
                <meta charset="utf-8" />
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1, user-scalable=0, maximum-scale=1.0, interactive-widget=resizes-content"
                />

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
