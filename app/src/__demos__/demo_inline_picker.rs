use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoInlinePicker() -> impl IntoView {
    view! {
        <Stylesheet href="/components/inline_picker.css" />

        <div class="mainDiv">
            <div class="inpTime">
                <svg
                    width="14"
                    height="14"
                    viewBox="0 0 14 14"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path d="M7 4V7" id="hourHand" />
                    <path d="M7 2.5V7" id="minHand" />
                </svg>
                <input type="time" value="09:41" id="timeInput" />
            </div>
        </div>

        <script src="/components/inline_picker.js"></script>
    }
}
