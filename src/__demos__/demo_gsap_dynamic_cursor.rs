use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoGsapDynamicCursor() -> impl IntoView {
    view! {
        <Stylesheet href="/components/dynamic_cursor.css" />

        <div class="mainDiv">
            <section class="cloneable">
                <div class="button-row">
                    <a
                        href="#"
                        data-cursor="Pretty cool, right?"
                        class="button text-neutral-800 bg-light"
                    >
                        <p class="button-text">{"A GSAP-powered custom cursor"}</p>
                        <div class="button-bg"></div>
                    </a>
                </div>

                <div class="cursor">
                    <p class="text-neutral-300">Learn more</p>
                </div>
            </section>
        </div>

        // TODO. Harmonize all the scripts for gsap. Better to have in shell but probably in conflict with other Components so I keep it there for the moment.
        <script src="https://cdn.jsdelivr.net/npm/gsap@3.12.5/dist/gsap.min.js"></script>

        <script src="/components/dynamic_cursor.js"></script>
    }
}
