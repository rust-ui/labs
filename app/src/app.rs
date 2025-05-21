use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::{
    all_demos_page::AllDemosPage, gsap_intro_page::GsapIntroPage,
    shared::components::navbar::Navbar,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rust_ui_labs.css" />
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
