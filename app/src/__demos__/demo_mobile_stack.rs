use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoMobileStack() -> impl IntoView {
    view! {
        <Stylesheet href="/components/mobile_stack.css" />

        <div class="mainHtml">

            <div class="grid m-0 mainDiv place-content-center">

                <div id="grid">
                    {(0..10)
                        .map(|i| {
                            let icons = [
                                "☰",
                                "👍",
                                "👎",
                                "🎸",
                                "🍽️",
                                "🍈",
                                "🥓",
                                "🪜",
                                "🎥",
                                "🐟",
                            ];
                            view! {
                                <div style=format!(
                                    "view-transition-name: item-{}; --sibling-index: {};",
                                    i,
                                    i,
                                )>{icons[i]}</div>
                            }
                        })
                        .collect_view()}
                </div>

            </div>

        </div>

        <script src="/components/mobile_stack.js"></script>
    }
}
