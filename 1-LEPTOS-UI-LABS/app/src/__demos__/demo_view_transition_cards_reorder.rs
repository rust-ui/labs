use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoViewTransitionCardsReorder() -> impl IntoView {
    view! {
        <Stylesheet href="/components/view_transition_cards_reorder.css" />

        <div class="cards flex justify-center">
        <div class="card" id="card-1">1</div>
        <div class="card" id="card-2">2</div>
        <div class="card" id="card-3">3</div>
        <div class="card" id="card-4">4</div>
        </div>

        <div class="flex justify-center">
            <button class="p-4 rounded-lg bg-neutral-100 text-gray-600">
                Reorder
            </button>
        </div>

        <div class="warning">
        <p>You browser does not support advanced <code>attr()</code>. As a result, this demo wont do anything. Try Chrome 133+.</p>
        </div>

        <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
        <script src="/components/view_transition_cards_reorder.js"></script>
    }
}
