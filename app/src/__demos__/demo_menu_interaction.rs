use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoMenuInteraction() -> impl IntoView {
    view! {
        <Stylesheet href="/components/menu_interaction.css" />

        <div class="mainDiv">

            <div class="widget">
                <div class="menu">
                    <div class="toggle">
                        <i></i>
                        <i></i>
                    </div>
                    <ul class="list">
                        <li></li>
                        <li></li>
                        <li></li>
                        <li></li>
                        <li></li>
                        <li></li>
                    </ul>
                </div>
            </div>

            <a
                class="dribbble"
                href="https://dribbble.com/shots/7197834-Menu-Interaction-Concept"
                target="_blank"
            >
                <img
                    src="https://dribbble.com/assets/logo-small-2x-9fe74d2ad7b25fba0f50168523c15fda4c35534f9ea0b1011179275383035439.png"
                    alt=""
                />
            </a>

        </div>

        <script src="/components/menu_interaction.js"></script>
    }
}
