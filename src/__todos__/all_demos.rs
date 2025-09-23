use leptos::prelude::*;

use crate::__TODOS__::demo_css_carousel::DemoCssCarousel;
use crate::__TODOS__::demo_pagination::DemoPagination;
use crate::__ready_to_port__::demo_tooltip::DemoTooltip;

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
        name: "👉 TODO: Pagination",
        render_fn: || view! { <DemoPagination /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: CSS Carousel",
        render_fn: || view! { <DemoCssCarousel /> }.into_any(),
    },
    DemoItem {
        name: "Tooltip",
        render_fn: || view! { <DemoTooltip /> }.into_any(),
    },
];
