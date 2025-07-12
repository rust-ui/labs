use leptos::prelude::*;
use leptos_meta::Stylesheet;

// TODO. Add cursor-pointer.

#[component]
pub fn DemoSliderPoint() -> impl IntoView {
    view! {
        <Stylesheet href="/components/slider_point.css" />

        <div class="mainDiv">
            <div class="points">
                <label for="p1" class="point">
                    <input id="p1" type="radio" name="points" checked />
                </label>
                <label for="p2" class="point">
                    <input id="p2" type="radio" name="points" />
                </label>

                <label for="p3" class="point">
                    <input id="p3" type="radio" name="points" />
                </label>
                <label for="p4" class="point">
                    <input id="p4" type="radio" name="points" />
                </label>
            </div>
        </div>

        <script src="/components/slider_point.js"></script>
    }
}
