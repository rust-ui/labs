use leptos::prelude::*;

use crate::__TODOS__::demo_css_carousel::DemoCssCarousel;
use crate::__TODOS__::demo_pagination::DemoPagination;
use crate::__ready_to_port__::demo_tree_view::DemoTreeView;
use crate::__ready_to_port__::demo_tree_view_renderer::DemoTreeViewShow;
use crate::__ready_to_port__::demo_tree_view_highlight::DemoTreeViewHighlight;

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
        name: "✅ Tree View",
        render_fn: || view! { <DemoTreeView /> }.into_any(),
    },
    DemoItem {
        name: "✅ Tree View Show",
        render_fn: || view! { <DemoTreeViewShow /> }.into_any(),
    },
    DemoItem {
        name: "✅ Tree View Highlight",
        render_fn: || view! { <DemoTreeViewHighlight /> }.into_any(),
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
