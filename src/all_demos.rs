use leptos::prelude::*;

use crate::__demos__::demo_alert_dialog::DemoAlertDialog;
use crate::__demos__::demo_apple_liquid_glass_ui::DemoAppleLiquidGlassUi;
use crate::__demos__::demo_button_multi_state::DemoButtonMultiState;
use crate::__demos__::demo_carousel::DemoCarousel;
use crate::__demos__::demo_carousel_hover_smooth::DemoCarouselHoverSmooth;

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
        name: "👉 TODO: Apple Liquid Glass UI",
        render_fn: || view! { <DemoAppleLiquidGlassUi /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Carousel",
        render_fn: || view! { <DemoCarousel /> }.into_any(),
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
];
