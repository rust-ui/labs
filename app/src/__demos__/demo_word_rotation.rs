use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::clx;

// TODO. Use a component for span.
// TODO. Pass the text color directly in the view! macro.
// TODO. Be able to pass the words in the script instead of hardcoded in the js file.

#[component]
pub fn DemoWordRotation() -> impl IntoView {
    view! {
        <Stylesheet href="/components/word_rotation.css" />

        <div class="h-[400px] grid place-content-center font-roboto-slab">
            <WordRotationWrapper>
                <WordRotation>
                    <span>Know</span>
                    <span style="view-transition-name: swap;">Productivity</span>
                    <span>with no limits.</span>
                </WordRotation>
            </WordRotationWrapper>
        </div>

        // ----- SCRIPT -----
        <script src="/components/word_rotation.js"></script>
    }
}

mod components {
    use super::*;
    clx! {WordRotationWrapper, main, "my-0 mx-auto py-8 px-4 w-[800px] resize text-[2.5rem] overflow-hidden"}
    clx! {WordRotation, p, "flex flex-wrap m-0 gap-[0.5ch] leading-[0.9]"}
}

pub use components::*;

// TODO. Add this to the WordRotation component.
// span:nth-of-type(2) {
//     color: hsl(64, 100%, 50%);
// }
