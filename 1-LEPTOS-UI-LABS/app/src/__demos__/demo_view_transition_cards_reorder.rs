use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::clx;
use rand::seq::SliceRandom;
use rand::thread_rng;

mod components {
    use super::*;
    clx! {Cards, div, "cards"}
    clx! {Card, div, "card"}
}

pub use components::*;

// Type for card data
#[derive(Debug, Clone)]
struct Card {
    id: &'static str,
}

fn shuffle_cards<T: Clone>(items: Vec<T>) -> Vec<T> {
    let mut rng = thread_rng();
    let mut shuffled = items.clone();
    shuffled.shuffle(&mut rng);
    shuffled
}

#[component]
pub fn DemoViewTransitionCardsReorder() -> impl IntoView {
    let cards = vec![
        Card { id: "1" },
        Card { id: "2" },
        Card { id: "3" },
        Card { id: "4" },
    ];

    view! {
        <Stylesheet href="/components/view_transition_cards_reorder.css" />

        <div class="grid place-items-center min-h-screen bg-[#212121] text-[#000]">
            <Cards>
                {shuffle_cards(cards.clone()).into_iter().map(|card| {
                    view! {
                        <Card>
                            <h3>{card.id}</h3>
                        </Card>
                    }
                }).collect::<Vec<_>>()}
            </Cards>
            <button class="p-4 rounded-lg bg-neutral-100">
                Reorder
            </button>
        </div>

        <div class="warning">
            <p>You browser does not support advanced <code>attr()</code>. As a result, this demo won 't do anything. Try Chrome 133+.</p>
        </div>

        // * ----- SCRIPT -----
        <script src="/components/view_transition_cards_reorder.js"></script>
    }
}
