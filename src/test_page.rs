use icons::PlusAnimate;
use leptos::prelude::*;

#[component]
pub fn TestPage() -> impl IntoView {
    view! {
        <div>
            <h1>Test Page</h1>
            <PlusAnimate />
        </div>
    }
}
