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
            <MultiRangeSlider />
            <DemoTest />
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
