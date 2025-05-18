use leptos::prelude::*;

use crate::__demos__::demo_alert_dialog::DemoAlertDialog;
use crate::__demos__::demo_bottom_bar_awwards::DemoBottomBarAwwwards;
use crate::__demos__::demo_card_reorder::DemoCardReorder;
use crate::__demos__::demo_carousel::DemoCarousel;
use crate::__demos__::demo_carousel_hover_smooth::DemoCarouselHoverSmooth;
use crate::__demos__::demo_drawer::DemoDrawer;
use crate::__demos__::demo_gsap_dynamic_cursor::DemoGsapDynamicCursor;
use crate::__demos__::demo_gsap_looping_words::DemoGsapLoopingWords;
use crate::__demos__::demo_inline_picker::DemoInlinePicker;
use crate::__demos__::demo_menu_bar_interaction::DemoMenuBarInteraction;
use crate::__demos__::demo_menu_interaction::DemoMenuInteraction;

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
        name: "👉 TODO: Gsap Dynamic Cursor",
        render_fn: || view! { <DemoGsapDynamicCursor /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Carousel",
        render_fn: || view! { <DemoCarousel /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Drawer",
        render_fn: || view! { <DemoDrawer /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Alert Dialog",
        render_fn: || view! { <DemoAlertDialog /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Card Reorder",
        render_fn: || view! { <DemoCardReorder /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Bottom Bar Awwwards",
        render_fn: || view! { <DemoBottomBarAwwwards /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Carousel Hover Smooth",
        render_fn: || view! { <DemoCarouselHoverSmooth /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Gsap Looping Words",
        render_fn: || view! { <DemoGsapLoopingWords /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Inline Picker",
        render_fn: || view! { <DemoInlinePicker /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Menu Interaction",
        render_fn: || view! { <DemoMenuInteraction /> }.into_any(),
    },
    DemoItem {
        name: "👉 TODO: Menu Bar Interaction",
        render_fn: || view! { <DemoMenuBarInteraction /> }.into_any(),
    },
];
