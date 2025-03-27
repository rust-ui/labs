use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::clx;
use tw_merge::*;

#[component]
pub fn PageTestGeneral() -> impl IntoView {
    view! {
        <div>
            <h1>Page Test General</h1>
            <SearchInputDynamic />
            <DemoTest />
            <DemoCarousel />
        </div>
    }
}

#[component]
pub fn SearchInputDynamic() -> impl IntoView {
    view! {
        <script src="/components/search_input_dynamic.js" />

        <div class="flex items-center justify-center h-[300px] bg-white">
            <div class="relative w-32 transition-all">
                <input
                    type="text"
                    placeholder="Arama.."
                    id="searchInput"
                    class="w-full py-2 pl-10 pr-4 text-gray-700 transition-all bg-white border border-gray-300 rounded-full outline-none"
                />
                <div class="absolute inset-y-0 left-0 flex items-center pl-3">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="w-5 h-5 text-gray-400 transition-all"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                        />
                    </svg>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn DemoCarousel() -> impl IntoView {
    view! {
        // * https://tailwindflex.com/@nikolai-petrovich/image-carousel-2
        <script src="/components/carousel.js" />

        <div class="relative">
            <div class="flex max-w-xl carousel">

                <div class="carousel-item">
                    <img
                        src="https://source.unsplash.com/random/800x600"
                        alt="Carousel Image 1"
                        class="object-cover w-full h-96"
                    />
                </div>
                <div class="carousel-item">
                    <img
                        src="https://source.unsplash.com/random/800x600?2"
                        alt="Carousel Image 2"
                        class="object-cover w-full h-96"
                    />
                </div>
                <div class="carousel-item">
                    <img
                        src="https://source.unsplash.com/random/800x600?3"
                        alt="Carousel Image 3"
                        class="object-cover w-full h-96"
                    />
                </div>
            </div>

            // CAROUSEL CONTROLS
            <div class="absolute inset-y-0 left-0 flex items-center justify-start pl-4">
                <button class="p-2 text-white bg-gray-800 rounded-full carousel-control-prev hover:bg-gray-700 focus:outline-none">
                    <svg
                        class="w-6 h-6"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M15 19l-7-7 7-7"
                        ></path>
                    </svg>
                </button>
            </div>
            <div class="absolute inset-y-0 right-0 flex items-center justify-end pr-4">
                <button class="p-2 text-white bg-gray-800 rounded-full carousel-control-next hover:bg-gray-700 focus:outline-none">
                    <svg
                        class="w-6 h-6"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M9 5l7 7-7 7"
                        ></path>
                    </svg>
                </button>
            </div>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn DemoTest() -> impl IntoView {
    view! {
        <div class="flex overflow-hidden relative justify-center items-center p-20 w-full h-full rounded-lg border md:shadow-xl min-h-[300px] bg-background">
            <p class="z-10 text-5xl font-medium tracking-tighter text-center whitespace-pre-wrap">
                Retro
            </p>
            <BackgroundRetro>
                <BackgroundRetroGrid />
                <div class="absolute inset-0 bg-gradient-to-t from-white to-transparent dark:from-black to-90%" />
            </BackgroundRetro>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

mod components {
    use super::*;
    clx! {BackgroundRetro, div, "absolute w-full h-full opacity-50 [perspective:200px]"}
}

pub use components::*;

use crate::components::ui::_animations::ANIMATIONS;

#[component]
pub fn BackgroundRetroGrid(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    let class = Memo::new(move |_| {
        tw_merge!(
            ANIMATIONS.BACKGROUND_RETRO_GRID,
            "[background-image:linear-gradient(to_right,rgba(0,0,0,0.3)_1px,transparent_0),linear-gradient(to_bottom,rgba(0,0,0,0.3)_1px,transparent_0)] [background-repeat:repeat] [background-size:60px_60px] [height:300vh] [inset:0%_0px] [margin-left:-50%] [transform-origin:100%_0_0] [width:600vw] dark:[background-image:linear-gradient(to_right,rgba(255,255,255,0.2)_1px,transparent_0),linear-gradient(to_bottom,rgba(255,255,255,0.2)_1px,transparent_0)]",
            class()
        )
    });

    view! {
        <div class="absolute inset-0 [transform:rotateX(35deg)]">
            <div class=class />
        </div>
    }
}
