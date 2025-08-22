use leptos::prelude::*;

use crate::__TODOS__::demo_alert_dialog::DemoAlertDialog;
use crate::__TODOS__::demo_apple_liquid_glass_ui::DemoAppleLiquidGlassUi;
use crate::__TODOS__::demo_bento3_transition::DemoBento3Transition;
use crate::__TODOS__::demo_button_multi_state::DemoButtonMultiState;
use crate::__TODOS__::demo_card_reverse::DemoCardReverse;
use crate::__TODOS__::demo_carousel_hover_smooth::DemoCarouselHoverSmooth;
use crate::__TODOS__::demo_pagination::DemoPagination;

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
        name: "👉 TODO:  Card Reverse",
        render_fn: || view! { <DemoCardReverse /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Pagination",
        render_fn: || view! { <DemoPagination /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Apple Liquid Glass UI",
        render_fn: || view! { <DemoAppleLiquidGlassUi /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Alert Dialog",
        render_fn: || view! { <DemoAlertDialog /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Carousel Hover Smooth",
        render_fn: || view! { <DemoCarouselHoverSmooth /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Button Multi State",
        render_fn: || view! { <DemoButtonMultiState /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Bento3 Transition",
        render_fn: || view! { <DemoBento3Transition /> }.into_any(),
    },
];
