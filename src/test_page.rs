use leptos::prelude::*;

use crate::__ready_to_port__::demo_login_page_1::DemoLoginPage1;

#[component]
pub fn TestPage() -> impl IntoView {
    view! {
        <div>
            <DemoLoginPage1 />
        </div>
    }
}
