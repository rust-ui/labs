use icons::{HeartAnimate, PlusAnimate};
use leptos::prelude::*;
use leptos_ui::clx;

#[component]
pub fn TestPage() -> impl IntoView {
    clx! {IconsWrapper, div, "group", "flex justify-center items-center p-2 rounded-md transition-colors duration-200 cursor-pointer select-none  hover:bg-accent"}

    view! {
        <div>
            <h1>Test Page</h1>

            <div class="flex gap-8 justify-center items-center m-0 min-h-screen font-sans">
                <IconsWrapper>
                    <HeartAnimate />
                </IconsWrapper>

                <PlusAnimate />
            </div>
        </div>
    }
}
