use icons::{AArrowDownAnimate, HeartAnimate, PlusAnimate};
use leptos::prelude::*;

#[component]
pub fn TestPage() -> impl IntoView {
    view! {
        <div>
            <h1>Test Page</h1>

            <div class="flex gap-8 justify-center items-center m-0 min-h-screen font-sans">
                <AArrowDownAnimate />
                <HeartAnimate />
                <PlusAnimate />
            </div>
        </div>
    }
}
