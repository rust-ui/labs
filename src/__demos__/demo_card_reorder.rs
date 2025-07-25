use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoCardReorder() -> impl IntoView {
    view! {
        <Stylesheet href="/components/card_reorder.css" />

        <div class="flex justify-center flex-row gap-4 p-4">
            <div class="rounded bg-gray-300 grid place-content-center text-2xl card" id="card-1">
                1
            </div>
            <div class="rounded bg-gray-300 grid place-content-center text-2xl card" id="card-2">
                2
            </div>
            <div class="rounded bg-gray-300 grid place-content-center text-2xl card" id="card-3">
                3
            </div>
            <div class="rounded bg-gray-300 grid place-content-center text-2xl card" id="card-4">
                4
            </div>
        </div>

        <div class="flex justify-center">
            <button class="p-4 text-gray-600 rounded-lg bg-neutral-100">Reorder</button>
        </div>

        <div class="p-4 my-4 border border-gray-300 text-center fixed bottom-0 left-4 right-4 warning">
            <p>
                You browser does not support advanced <code>attr()</code>
                . As a result, this demo wont do anything. Try Chrome 133+.
            </p>
        </div>

        <script src="/components/card_reorder.js"></script>
    }
}
