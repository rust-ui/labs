use leptos::prelude::*;
use leptos_ui::clx;

use crate::components::ui::button::Button;

#[component]
pub fn DemoCardReorder() -> impl IntoView {
    clx! {Card, div, "grid place-content-center text-2xl bg-gray-300 rounded card w-[20vw] max-w-20 aspect-[1/1.6]"}

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
            <Card id="card-1" style="view-transition-name: card-1; view-transition-class: card;">
                1
            </Card>
            <Card id="card-2" style="view-transition-name: card-2; view-transition-class: card;">
                2
            </Card>
            <Card id="card-3" style="view-transition-name: card-3; view-transition-class: card;">
                3
            </Card>
            <Card id="card-4" style="view-transition-name: card-4; view-transition-class: card;">
                4
            </Card>
        </div>

        <div class="flex justify-center">
            <Button>Reorder</Button>
        </div>

        <script src="/components/card_reorder.js"></script>
    }
}
