use icons::{AArrowDownAnimate, HeartAnimate, PlusAnimate};
use leptos::prelude::*;

#[component]
pub fn TestPage() -> impl IntoView {
    view! {
        <div>
            <h1>Test Page</h1>

            <div class="flex gap-8 justify-center items-center m-0 min-h-screen font-sans">
                <div class="flex justify-center items-center p-2 rounded-md transition-colors duration-200 cursor-pointer select-none hover:bg-accent">
                    <AArrowDownAnimate />
                </div>

                <HeartAnimate />
                <PlusAnimate />
            </div>
        </div>
    }
}
