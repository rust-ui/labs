use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::{all_demos_page::AllDemosPage, shared::components::navbar::Navbar};

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
                    <Route path=StaticSegment("/") view=AllDemosPage />
                </Routes>
            </div>
        </Router>
    }
}
