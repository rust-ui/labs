use leptos::prelude::*;

use crate::components::demos::{demo_tabs::DemoTabs, demo_tabs_responsive::DemoTabsResponsive};

#[component]
pub fn PageHome() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 m-4">
            <h1>"Welcome to Leptos!"</h1>

            <DemoTabsResponsive />
            <DemoTabs />
        </div>
    }
}
