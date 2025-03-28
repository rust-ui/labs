use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment,
};

use crate::{
    routing::{
        layout_test::LayoutTest, page_demo_tailwind::PageDemoTailwind, page_home::PageHome,
        page_js_drawer::PageJsDrawer, page_js_grid_collection::PageJsGridCollection,
        page_js_need_help::PageJsNeedHelp, page_multi_select_tags::PageMultiSelectTags,
        page_payment_method::PagePaymentMethod, page_scroll_top_button::PageScrollTopButton,
        page_scroll_view_progress::PageScrollViewProgress, page_slot_picker::PageSlotPicker,
        page_tailwind_tabs::PageTailwindTabs, page_test_general::PageTestGeneral,
        page_vercel_interaction::PageVercelInteraction,
    },
    shared::components::navbar::Navbar,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css" />
        <Title text="Welcome to Leptos" />

        <Router>
            <Navbar />
            <div class="min-h-screen">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=PageHome />

                    <ParentRoute path=StaticSegment("/test") view=LayoutTest>
                        <Route path=StaticSegment("/general") view=PageTestGeneral />
                        <Route path=StaticSegment("/js-need-help") view=PageJsNeedHelp />
                        <Route
                            path=StaticSegment("/js-grid-collection")
                            view=PageJsGridCollection
                        />
                        <Route path=StaticSegment("/payment-method") view=PagePaymentMethod />
                        <Route
                            path=StaticSegment("/vercel-interaction")
                            view=PageVercelInteraction
                        />
                        <Route path=StaticSegment("/js-drawer") view=PageJsDrawer />
                        <Route path=StaticSegment("/tailwind-tabs") view=PageTailwindTabs />
                        <Route
                            path=StaticSegment("/scroll-view-progress")
                            view=PageScrollViewProgress
                        />
                        <Route path=StaticSegment("/scroll-top-button") view=PageScrollTopButton />
                        <Route path=StaticSegment("/demo-tailwind") view=PageDemoTailwind />
                        <Route path=StaticSegment("/multi-select-tags") view=PageMultiSelectTags />
                        <Route path=StaticSegment("/slot-picker") view=PageSlotPicker />
                    </ParentRoute>
                </Routes>
            </div>
        </Router>
    }
}
