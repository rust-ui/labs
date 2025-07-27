use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoCardReorder() -> impl IntoView {
    view! {
        <Stylesheet href="/components/card_reorder.css" />

        <div class="flex flex-row gap-4 justify-center p-4">
            <div class="grid place-content-center text-2xl bg-gray-300 rounded card" id="card-1">
                1
            </div>
            <div class="grid place-content-center text-2xl bg-gray-300 rounded card" id="card-2">
                2
            </div>
            <div class="grid place-content-center text-2xl bg-gray-300 rounded card" id="card-3">
                3
            </div>
            <div class="grid place-content-center text-2xl bg-gray-300 rounded card" id="card-4">
                4
            </div>
        </div>

        <div class="flex justify-center">
            <button class="p-4 text-gray-600 rounded-lg bg-neutral-100">Reorder</button>
        </div>

        <script src="/components/card_reorder.js"></script>
    }
}
