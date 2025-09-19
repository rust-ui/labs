use leptos::prelude::*;

use crate::__TODOS__::demo_button_stateful::DemoButtonStateful;
use crate::__TODOS__::demo_card_reverse::DemoCardReverse;
use crate::__TODOS__::demo_css_carousel::DemoCssCarousel;
use crate::__TODOS__::demo_pagination::DemoPagination;
use crate::__ready_to_port__::DemoChips;

#[derive(Clone)]
pub struct DemoItem {
    pub name: &'static str,
    pub render_fn: fn() -> AnyView,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub const ALL_DEMOS: &[DemoItem] = &[
    DemoItem {
        name: "✅ READY: Chips",
        render_fn: || view! { <DemoChips /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO:  Card Reverse",
        render_fn: || view! { <DemoCardReverse /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Pagination",
        render_fn: || view! { <DemoPagination /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: CSS Carousel",
        render_fn: || view! { <DemoCssCarousel /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Button Multi State",
        render_fn: || view! { <DemoButtonStateful /> }.into_any(),
    },
];
