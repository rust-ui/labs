use leptos::prelude::*;
use leptos_meta::Stylesheet;

use crate::shared::components::button::Button;

#[component]
pub fn DemoButtonMultiState() -> impl IntoView {
    view! {
        <Stylesheet href="/components/button_multi_state.css" />

        <div class="mainDiv">
            <Button id="demo">Do some hard work</Button>
        </div>

        <script src="/components/button_multi_state.js"></script>
    }
}
