use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::{
    all_demos_page::AllDemosPage, gsap_intro_page::GsapIntroPage,
    shared::components::navbar::Navbar,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css" />
        <Title text="Lepos UI Labs" />

        <Router>
            <Navbar />
            <div class="min-h-screen">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route
                        path=StaticSegment("")
                        view=move || {
                            view! { <AllDemosPage /> }
                        }
                    />
                    <Route path=StaticSegment("/gsap-intro") view=GsapIntroPage />
                </Routes>
            </div>
        </Router>
    }
}
