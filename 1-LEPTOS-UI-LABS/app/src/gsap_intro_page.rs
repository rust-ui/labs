use leptos::prelude::*;

#[component]
pub fn GsapIntroPage() -> impl IntoView {
    view! {
        <style>
            {"::-webkit-scrollbar {
            display: none;
            }
            
            .img__holder {
            clip-path: polygon(37.5% 20%, 62.5% 20%, 62.5% 80%, 37.5% 80%);
            }
            "}
        </style>

        <div class="h-full mainDiv overscroll-none font-inte">

            <div class="fixed flex flex-row w-full transform -translate-x-1/2 -translate-y-1/2 top-1/2 left-1/2 header__scroll z-2">
                <div class="flex flex-1 letters">
                    <div class="flex-1 text-center text-[18vw]">"T"</div>
                    <div class="flex-1 text-center text-[18vw]">"A"</div>
                    <div class="flex-1 text-center text-[18vw]">"I"</div>
                    <div class="flex-1 text-center text-[18vw]">"L"</div>
                </div>
                <div class="flex flex-1 text-center text-[18vw] letters">
                    <div class="flex-1 text-center text-[18vw]">"W"</div>
                    <div class="flex-1 text-center text-[18vw]">"I"</div>
                    <div class="flex-1 text-center text-[18vw]">"N"</div>
                    <div class="flex-1 text-center text-[18vw]">"D"</div>
                </div>
            </div>
            <div class="fixed top-0 w-full min-h-screen website-content">
                <div class="img__holder relative w-full h-[100vh] top-0 bg-white rotate-[30deg]">
                    <img
                        class="object-cover w-full h-full relative scale-[2]"
                        src="https://images.unsplash.com/photo-1700403322391-f1cd144394cb?q=80&w=2070&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                        alt=""
                    />
                </div>

                <div class="relative bg-muted/60 p-1 -w-full content__holder min-h-[100vh] grid place-content-center">
                    <h2 class="font-bold text-4xl">"Your content goes here 😄"</h2>
                </div>
            </div>

        </div>

        <script src="/components/gsap_intro.js" />
    }
}
