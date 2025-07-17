use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::{
    components::navbar::Navbar,
    routing::{page_home::HomePage, page_test::TestPage},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start_leptos_ssr.css" />
        <Title text="Rust UI Starters - Leptos SSR" />

        <Router>
            <Navbar />
            <div class="min-h-screen">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=HomePage />
                    <Route path=StaticSegment("/test") view=TestPage />
                </Routes>
            </div>
        </Router>
    }
}
