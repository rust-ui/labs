use leptos::prelude::*;

use crate::__demos__::demo_alert_dialog::DemoAlertDialog;
use crate::__demos__::demo_card_reorder::DemoCardReorder;
use crate::__demos__::demo_carousel::DemoCarousel;
use crate::__demos__::demo_drawer::DemoDrawer;
use crate::__demos__::demo_gsap_dynamic_cursor::DemoGsapDynamicCursor;

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
        name: "Slot_1",
        render_fn: || view! { "SLOT_1" }.into_any(),
    },
    DemoItem {
        name: "Carousel",
        render_fn: || view! { <DemoCarousel /> }.into_any(),
    },
    DemoItem {
        name: "Drawer",
        render_fn: || view! { <DemoDrawer /> }.into_any(),
    },
    DemoItem {
        name: "Slot_4",
        render_fn: || view! { "SLOT_4" }.into_any(),
    },
    DemoItem {
        name: "Slot_5",
        render_fn: || view! { "SLOT_5" }.into_any(),
    },
    DemoItem {
        name: "Slot_6",
        render_fn: || view! { "SLOT_6" }.into_any(),
    },
    DemoItem {
        name: "Slot_7",
        render_fn: || view! { "SLOT_7" }.into_any(),
    },
    DemoItem {
        name: "Alert Dialog",
        render_fn: || view! { <DemoAlertDialog /> }.into_any(),
    },
    DemoItem {
        name: "Card Reorder",
        render_fn: || view! { <DemoCardReorder /> }.into_any(),
    },
    DemoItem {
        name: "Demo Gsap Dynamic Cursor",
        render_fn: || view! { <DemoGsapDynamicCursor /> }.into_any(),
    },
    DemoItem {
        name: "Slot_11",
        render_fn: || view! { "SLOT_11" }.into_any(),
    },
    DemoItem {
        name: "Slot_12",
        render_fn: || view! { "SLOT_12" }.into_any(),
    },
    DemoItem {
        name: "Slot_13",
        render_fn: || view! { "SLOT_13" }.into_any(),
    },
    DemoItem {
        name: "Slot_14",
        render_fn: || view! { "SLOT_14" }.into_any(),
    },
    DemoItem {
        name: "Slot_15",
        render_fn: || view! { "SLOT_15" }.into_any(),
    },
];
