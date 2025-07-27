use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoMobileStack() -> impl IntoView {
    view! {
        <Stylesheet href="/components/mobile_stack.css" />

        <div class="mainHtml">

            <div class="grid place-content-center m-0 mainDiv">

                <div id="grid">
                    <div style="view-transition-name: item-0; --sibling-index: 0;">{"☰"}</div>
                    <div style="view-transition-name: item-1; --sibling-index: 1;">{"👍"}</div>
                    <div style="view-transition-name: item-2; --sibling-index: 2;">{"👎"}</div>
                    <div style="view-transition-name: item-3; --sibling-index: 3;">{"🎸"}</div>
                    <div style="view-transition-name: item-4; --sibling-index: 4;">{"🍽️"}</div>
                    <div style="view-transition-name: item-5; --sibling-index: 5;">{"🍈"}</div>
                    <div style="view-transition-name: item-6; --sibling-index: 6;">{"🥓"}</div>
                    <div style="view-transition-name: item-7; --sibling-index: 7;">{"🪜"}</div>
                    <div style="view-transition-name: item-8; --sibling-index: 8;">{"🎥"}</div>
                    <div style="view-transition-name: item-9; --sibling-index: 9;">{"🐟"}</div>
                </div>

            </div>

        </div>

        <script src="/components/mobile_stack.js"></script>
    }
}
