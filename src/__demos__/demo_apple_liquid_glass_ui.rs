use leptos::prelude::*;
use leptos_meta::Stylesheet;

// Aknowledgement: https://codepen.io/joshua_k_y/pen/zYzQZqy

#[component]
pub fn DemoAppleLiquidGlassUi() -> impl IntoView {
    view! {
        <Stylesheet href="/components/apple_liquid_glass_ui.css" />

        <div class="flex flex-col justify-center items-center py-8 m-0 mainDiv">
            <div class="flex flex-row flex-col justify-center items-center liquid__glass__container liquid__glass__container__inline">
                <div class="flex overflow-hidden relative m-2 font-semibold text-white bg-transparent cursor-pointer glass__container glass_container__rounded min-w-[32rem] rounded-[5rem]">
                    <div class="glass__filter"></div>
                    <div class="glass__overlay bg-white/25"></div>
                    <div class="glass__specular"></div>
                    <div class="flex relative z-30 flex-1 gap-5 justify-between items-center py-1 px-6 pt-4 pr-8 pb-4 pl-3 glass__content">
                        <div class="flex flex-1 justify-between items-center w-full player">
                            <div class="flex justify-center items-center ml-2 player__thumb">
                                <img
                                    class="my-1 w-20 h-auto rounded-lg player__img"
                                    src="https://images.unsplash.com/photo-1619983081593-e2ba5b543168?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3wzMjM4NDZ8MHwxfHJhbmRvbXx8fHx8fHx8fDE3NDk1NzAwNDV8&ixlib=rb-4.1.0&q=80&w=400"
                                    alt="All Of Me"
                                />
                                <div class="flex flex-col mx-4 text-black player__legend">
                                    <h3 class="m-0 text-base player__legend__title">All Of Me</h3>
                                    <span class="m-0 text-base player__legend__sub-title opacity-45">
                                        Nao
                                    </span>
                                </div>
                            </div>
                            <div class="flex justify-center items-center -mr-4 player__controls">
                                <div class="flex mr-4 player__controls__play">
                                    <Svg_PlayIcon />
                                </div>
                                <div class="flex player__controls__ff">
                                    <Svg_FastForwardIcon />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex flex-row flex-col justify-center items-center liquid__glass__container liquid__glass__container__inline">
                <div class="flex overflow-hidden relative m-2 font-semibold text-white bg-transparent cursor-pointer glass__container glass_container__rounded rounded-[5rem]">
                    <div class="absolute inset-0 z-0 glass__filter"></div>
                    <div class="absolute inset-0 z-10 glass__overlay bg-white/25"></div>
                    <div class="overflow-hidden absolute inset-0 z-20 glass__specular"></div>
                    <div class="flex relative z-30 gap-5 items-center px-6 pt-4 pb-4 glass__content glass__content__alone">
                        <div class="flex flex-col justify-between items-center text-center text-white rounded-lg glass__item">
                            <Svg_SearchIcon />
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex flex-col justify-center items-center liquid__glass__container">
                <div class="flex overflow-hidden relative font-semibold text-white bg-transparent rounded-2xl cursor-pointer glass__container">
                    <div class="absolute inset-0 z-0 glass__filter"></div>
                    <div class="absolute inset-0 z-10 glass__overlay bg-white/25"></div>
                    <div class="overflow-hidden absolute inset-0 z-20 glass__specular"></div>
                    <div class="flex relative z-30 gap-5 items-center px-6 pt-4 pb-4 glass__content">
                        <a href="#" class="inline-block relative p-px">
                            <img
                                src="https://assets.codepen.io/923404/finder.png"
                                alt="Finder"
                                class="block hover:scale-95 w-[75px]"
                            />
                        </a>
                        <a href="#" class="inline-block relative p-px">
                            <img
                                src="https://assets.codepen.io/923404/map.png"
                                alt="Maps"
                                class="block hover:scale-95 w-[75px]"
                            />
                        </a>
                        <a href="#" class="inline-block relative p-px">
                            <img
                                src="https://assets.codepen.io/923404/messages.png"
                                alt="Messages"
                                class="block hover:scale-95 w-[75px]"
                            />
                        </a>
                        <a href="#" class="inline-block relative p-px">
                            <img
                                src="https://assets.codepen.io/923404/safari.png"
                                alt="Safari"
                                class="block hover:scale-95 w-[75px]"
                            />
                        </a>
                        <a href="#" class="inline-block relative p-px">
                            <img
                                src="https://assets.codepen.io/923404/books.png"
                                alt="Books"
                                class="block hover:scale-95 w-[75px]"
                            />
                        </a>
                    </div>

                    <Svg_LiquidGlassFilter />

                </div>
            </div>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn Svg_PlayIcon() -> impl IntoView {
    view! {
        <svg viewBox="0 0 448 512" width="24" title="play" class="fill-gray-500">
            <path d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z" />
        </svg>
    }
}

#[component]
pub fn Svg_FastForwardIcon() -> impl IntoView {
    view! {
        <svg viewBox="0 0 448 512" width="24" title="fast-forward" class="fill-gray-500">
            <path d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z" />
        </svg>
        <svg viewBox="0 0 448 512" width="24" title="fast-forward" class="fill-gray-500">
            <path d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z" />
        </svg>
    }
}

#[component]
pub fn Svg_SearchIcon() -> impl IntoView {
    view! {
        <svg viewBox="0 0 512 512" width="40" title="search" class="mb-1 fill-white h-[50px]">
            <path d="M505 442.7L405.3 343c-4.5-4.5-10.6-7-17-7H372c27.6-35.3 44-79.7 44-128C416 93.1 322.9 0 208 0S0 93.1 0 208s93.1 208 208 208c48.3 0 92.7-16.4 128-44v16.3c0 6.4 2.5 12.5 7 17l99.7 99.7c9.4 9.4 24.6 9.4 33.9 0l28.3-28.3c9.4-9.4 9.4-24.6.1-34zM208 336c-70.7 0-128-57.2-128-128 0-70.7 57.2-128 128-128 70.7 0 128 57.2 128 128 0 70.7-57.2 128-128 128z" />
        </svg>
    }
}

#[component]
pub fn Svg_LiquidGlassFilter() -> impl IntoView {
    view! {
        <svg style="display: none">
            <filter id="lg-dist" x="0%" y="0%" width="100%" height="100%">
                <feTurbulence
                    type="fractalNoise"
                    baseFrequency="0.008 0.008"
                    numOctaves="2"
                    seed="92"
                    result="noise"
                />
                <feGaussianBlur in="noise" stdDeviation="2" result="blurred" />
                <feDisplacementMap
                    in="SourceGraphic"
                    in2="blurred"
                    scale="70"
                    xChannelSelector="R"
                    yChannelSelector="G"
                />
            </filter>
        </svg>
    }
}
