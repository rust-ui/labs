use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::StaticSegment;
use leptos_router::components::{Route, Router, Routes};

use crate::__TODOS__::all_demos_page::AllDemosPage;
use crate::components::_app::navbar::Navbar;
use crate::gsap_intro_page::GsapIntroPage;
use crate::page_test::PageTest;

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
                    <Route path=StaticSegment("/test") view=PageTest />
                    <Route path=StaticSegment("/gsap-intro") view=GsapIntroPage />
                </Routes>
            </div>
        </Router>
    }
}
