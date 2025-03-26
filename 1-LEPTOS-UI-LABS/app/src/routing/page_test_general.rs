use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::demos::demo_otp::DemoOtp;

#[component]
pub fn PageTestGeneral() -> impl IntoView {
    view! {
        <div>
            <h1>Page Test General</h1>
            <DemoOtp />
            <DemoRangeSlider />
            <MultiRangeSlider />
            <DemoTest />
            <DemoCarousel />
            <DemoChartJs />
        </div>
    }
}

#[component]
pub fn DemoChartJs() -> impl IntoView {
    view! {
        <script src="/components/chart_js.js" />

        <div class="w-full max-w-4xl p-6 bg-white shadow-lg rounded-2xl">
            <div class="flex items-center justify-between mb-4">
                <h1 class="text-xl font-semibold text-gray-800">Order Report</h1>
                <div class="flex items-center space-x-2">
                    <span class="text-sm text-gray-600">12, Mar 24 - 31, Mar 24</span>
                    <button class="px-2 py-1 text-white bg-blue-500 rounded-lg hover:bg-blue-600">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-5 h-5"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M16.5 9.5L12 14l-4.5-4.5"
                            />
                        </svg>
                    </button>
                </div>
            </div>
            <canvas id="orderReportChart" class="w-full h-80"></canvas>
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

#[component]
pub fn DemoRangeSlider() -> impl IntoView {
    view! {
        <script src="/components/range_slider.js" />

        <div class="w-full max-w-md p-6 bg-white rounded-lg shadow-lg">
            <h2 class="mb-4 text-2xl font-bold">Range Slider</h2>
            <div class="mb-4">
                <label for="price-range" class="block mb-2 font-bold text-gray-700">
                    Price Range
                </label>
                <input
                    type="range"
                    id="price-range"
                    class="w-full accent-indigo-600"
                    min="0"
                    max="1000"
                    value="500"
                    oninput="updatePrice(this.value)"
                />
            </div>
            <div class="flex justify-between text-gray-500">
                <span id="minPrice">$0</span>
                <span id="maxPrice">$1000</span>
            </div>
        </div>
    }
}

#[component]
pub fn MultiRangeSlider() -> impl IntoView {
    view! {
        <Stylesheet href="/components/multi-range-slider.css" />
        <script src="/components/multi-range-slider.js" />

        <div class="flex items-center justify-center min-h-screen bg-gray-100">
            <div class="w-[400px] bg-white p-6 rounded-lg shadow-lg">
                <h2 class="text-lg font-bold">PRICE RANGE</h2>

                <div class="relative mt-4 slider-container">

                    <input type="range" id="minRange" min="0" max="400" value="0" />
                    <input type="range" id="maxRange" min="0" max="400" value="400" />

                    <div class="relative w-full h-2 bg-gray-200 rounded-md">
                        <div
                            id="rangeTrack"
                            class="absolute h-2 rounded-md bg-gradient-to-r from-blue-900 to-blue-400"
                        ></div>
                    </div>
                </div>

                <div class="flex justify-between mt-3 text-gray-600">
                    <span>Min Price: $<span id="minValue">0</span></span>
                    <span>Max Price: $<span id="maxValue">400</span></span>
                </div>
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
