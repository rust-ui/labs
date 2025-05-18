use leptos::prelude::*;
use leptos_meta::Stylesheet;

// TODO. Add Button

#[component]
pub fn DemoButtonMultiState() -> impl IntoView {
    view! {
        <Stylesheet href="/components/button_multi_state.css" />

        <div class="mainDiv">
            <button id="demo">Do some hard work</button>
        </div>

        <script src="/components/button_multi_state.js"></script>
    }
}
