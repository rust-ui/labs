use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::{clx};

#[component]
pub fn DemoWordRotation() -> impl IntoView {
    view! {
        <Stylesheet href="/components/word_rotation.css" />

        <div class="m-0 grid content-center pl-4 min-h-screen font-['Roboto_Slab'] bg-[color:hsl(0,_0%,_0%)] text-[color:hsl(0,_0%,_98%)]">
            <Main>
                <Paragraph class="flex flex-wrap m-0 gap-[0.5ch] leading-[0.9]">
                    <span class="py-4 px-0">Know </span>
                    <span class="py-4 px-0">Productivity </span>
                    <span class="py-4 px-0"> with no limits.</span>
                </Paragraph>
            </Main>
        </div>

        // ----- SCRIPT -----
        <script src="/components/word_rotation.js"></script>
    }
}

mod components {
use super::*;
    clx! {Main, main, "my-0 mx-auto py-8 px-4 w-[800px] resize text-[2.5rem] overflow-hidden"}
    clx! {Paragraph, p, "flex flex-wrap m-0 gap-[0.5ch] leading-[0.9]"}
}

pub use components::*;