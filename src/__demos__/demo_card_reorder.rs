use leptos::prelude::*;

#[component]
pub fn DemoCardReorder() -> impl IntoView {
    view! {
        <style>
            r#"
            :root {
            view-transition-name: none;
            }
            
            ::view-transition {
            pointer-events: none;
            }
            "#
        </style>

        <div class="flex flex-row gap-4 justify-center p-4">
            <div
                class="grid place-content-center text-2xl bg-gray-300 rounded card w-[20vw] max-w-20 aspect-[1/1.6]"
                id="card-1"
                style="view-transition-name: card-1; view-transition-class: card;"
            >
                1
            </div>
            <div
                class="grid place-content-center text-2xl bg-gray-300 rounded card w-[20vw] max-w-20 aspect-[1/1.6]"
                id="card-2"
                style="view-transition-name: card-2; view-transition-class: card;"
            >
                2
            </div>
            <div
                class="grid place-content-center text-2xl bg-gray-300 rounded card w-[20vw] max-w-20 aspect-[1/1.6]"
                id="card-3"
                style="view-transition-name: card-3; view-transition-class: card;"
            >
                3
            </div>
            <div
                class="grid place-content-center text-2xl bg-gray-300 rounded card w-[20vw] max-w-20 aspect-[1/1.6]"
                id="card-4"
                style="view-transition-name: card-4; view-transition-class: card;"
            >
                4
            </div>
        </div>

        <div class="flex justify-center">
            <button class="p-4 text-gray-600 rounded-lg bg-neutral-100">Reorder</button>
        </div>

        <script src="/components/card_reorder.js"></script>
    }
}
