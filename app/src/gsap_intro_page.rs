use leptos::prelude::*;
use leptos_ui::{clx, img};

#[component]
pub fn GsapIntroPage() -> impl IntoView {
    view! {
        <GsapIntro>
            <HeaderScroll>
                <Letters>
                    <Letter>"T"</Letter>
                    <Letter>"A"</Letter>
                    <Letter>"I"</Letter>
                    <Letter>"L"</Letter>
                </Letters>
                <Letters>
                    <Letter>"W"</Letter>
                    <Letter>"I"</Letter>
                    <Letter>"N"</Letter>
                    <Letter>"D"</Letter>
                </Letters>
            </HeaderScroll>

            <FixedContent>
                <ImageHolder>
                    <Image
                        src="https://images.unsplash.com/photo-1700403322391-f1cd144394cb?q=80&w=2070&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                        alt=""
                    />
                </ImageHolder>

                <ContentHolder class="grid place-content-center">
                    <h2 class="font-bold text-4xl">"Your content goes here 😄"</h2>
                </ContentHolder>
            </FixedContent>
        </GsapIntro>

        <style>
            {"::-webkit-scrollbar {
            display: none;
            }
            
            .img__holder {
            clip-path: polygon(37.5% 20%, 62.5% 20%, 62.5% 80%, 37.5% 80%);
            }
            "}
        </style>

        // * ----- SCRIPT -----
        <script src="/components/gsap_intro.js" />
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ COMPONENTS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

mod components {
    use super::*;
    clx! {GsapIntro, div, "h-full overscroll-none font-inte"}
    clx! {HeaderScroll, div, "header__scroll", "fixed flex flex-row w-full transform -translate-x-1/2 -translate-y-1/2 top-1/2 left-1/2 z-2"}
    clx! {Letters, div, "letters", "flex flex-1"}
    clx! {Letter, div, "flex-1 text-center text-[18vw]"}
    clx! {FixedContent, div, "fixed__content", "fixed top-0 w-full min-h-screen"}
    clx! {ImageHolder, div, "img__holder", "relative w-full h-[100vh] top-0 bg-white rotate-[30deg]"}
    clx! {ContentHolder, div, "content__holder", "relative bg-muted/60 p-1 -w-full min-h-[100vh]"}

    img! {Image, "object-cover w-full h-full relative scale-[2]"}
}

pub use components::*;
