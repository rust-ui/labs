use leptos::prelude::*;

use crate::__TODOS__::demo_css_carousel::DemoCssCarousel;
use crate::__TODOS__::demo_pagination::DemoPagination;
use crate::__ready_to_port__::demo_accordion::DemoAccordion;
use crate::__ready_to_port__::demo_accordion_bordered::DemoAccordionBordered;
use crate::__ready_to_port__::demo_accordion_icons::DemoAccordionIcons;

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
        name: "Accordion Icons",
        render_fn: || view! { <DemoAccordionIcons /> }.into_any(),
    },
    DemoItem {
        name: "Accordion",
        render_fn: || view! { <DemoAccordion /> }.into_any(),
    },
    DemoItem {
        name: "Accordion bordered",
        render_fn: || view! { <DemoAccordionBordered /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Pagination",
        render_fn: || view! { <DemoPagination /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: CSS Carousel",
        render_fn: || view! { <DemoCssCarousel /> }.into_any(),
    },
];
