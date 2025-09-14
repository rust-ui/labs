use icons::{AArrowDownAnimate, HeartAnimate, PlusAnimate};
use leptos::prelude::*;

#[component]
pub fn TestPage() -> impl IntoView {
    view! {
        <div>
            <h1>Test Page</h1>
            <AArrowDownAnimate />
            <HeartAnimate />
            <PlusAnimate />
        </div>
    }
}
