use leptos::prelude::*;
use leptos_meta::Stylesheet;

// Aknowledgement: https://codepen.io/joshua_k_y/pen/zYzQZqy

#[component]
pub fn DemoAppleLiquidGlassUi() -> impl IntoView {
    view! {
        <Stylesheet href="/components/apple_liquid_glass_ui.css" />

        <div class="mainDiv m-0 py-8 flex flex-col items-center justify-center">
            <div class="liquid__glass__container liquid__glass__container__inline flex flex-col items-center justify-center flex-row">
                <div class="glass__container glass_container__rounded min-w-[32rem] relative flex m-2 rounded-[5rem] font-semibold text-white cursor-pointer bg-transparent overflow-hidden">
                    <div class="glass__filter"></div>
                    <div class="glass__overlay"></div>
                    <div class="glass__specular"></div>
                    <div class="glass__content items-center gap-5 flex-1 justify-between relative z-30 flex px-6 pt-4 pb-4 py-1 pl-3 pr-8">
                        <div class="player items-center flex-1 justify-between flex w-full">
                            <div class="player__thumb items-center justify-center flex ml-2">
                                <img
                                    class="player__img w-20 h-auto rounded-lg my-1"
                                    src="https://images.unsplash.com/photo-1619983081593-e2ba5b543168?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3wzMjM4NDZ8MHwxfHJhbmRvbXx8fHx8fHx8fDE3NDk1NzAwNDV8&ixlib=rb-4.1.0&q=80&w=400"
                                    alt="All Of Me"
                                />
                                <div class="player__legend flex flex-col mx-4 text-black">
                                    <h3 class="player__legend__title text-base m-0">All Of Me</h3>
                                    <span class="player__legend__sub-title opacity-45 text-base m-0">Nao</span>
                                </div>
                            </div>
                            <div class="player__controls items-center justify-center flex -mr-4">
                                <div class="player__controls__play flex mr-4">
                                    <svg viewBox="0 0 448 512" width="24" title="play" class="fill-gray-500">
                                        <path
                                            d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z"
                                        />
                                    </svg>
                                </div>
                                <div class="player__controls__ff flex">
                                    <svg viewBox="0 0 448 512" width="24" title="play" class="fill-gray-500">
                                        <path
                                            d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z"
                                        />
                                    </svg>
                                    <svg viewBox="0 0 448 512" width="24" title="play" class="fill-gray-500">
                                        <path
                                            d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z"
                                        />
                                    </svg>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="liquid__glass__container liquid__glass__container__inline flex flex-col items-center justify-center flex-row">
                <div class="glass__container glass_container__rounded relative flex m-2 rounded-[5rem] font-semibold text-white cursor-pointer bg-transparent overflow-hidden">
                    <div class="glass__filter absolute inset-0 z-0"></div>
                    <div class="glass__overlay absolute inset-0 z-10"></div>
                    <div class="glass__specular absolute inset-0 z-20 overflow-hidden"></div>
                    <div class="glass__content items-center gap-5 glass__content__alone relative z-30 flex px-6 pt-4 pb-4">
                        <div class="glass__item items-center justify-between text-center flex flex-col rounded-lg text-white">
                            <svg viewBox="0 0 512 512" width="40" title="search" class="mb-1 fill-white h-[50px]">
                                <path d="M505 442.7L405.3 343c-4.5-4.5-10.6-7-17-7H372c27.6-35.3 44-79.7 44-128C416 93.1 322.9 0 208 0S0 93.1 0 208s93.1 208 208 208c48.3 0 92.7-16.4 128-44v16.3c0 6.4 2.5 12.5 7 17l99.7 99.7c9.4 9.4 24.6 9.4 33.9 0l28.3-28.3c9.4-9.4 9.4-24.6.1-34zM208 336c-70.7 0-128-57.2-128-128 0-70.7 57.2-128 128-128 70.7 0 128 57.2 128 128 0 70.7-57.2 128-128 128z" />
                            </svg>
                        </div>
                    </div>
                </div>
            </div>

            <div class="liquid__glass__container flex flex-col items-center justify-center">
                <div class="glass__container relative flex rounded-2xl font-semibold text-white cursor-pointer bg-transparent overflow-hidden">
                    <div class="glass__filter absolute inset-0 z-0"></div>
                    <div class="glass__overlay absolute inset-0 z-10"></div>
                    <div class="glass__specular absolute inset-0 z-20 overflow-hidden"></div>
                    <div class="glass__content items-center gap-5 relative z-30 flex px-6 pt-4 pb-4">
                        <a href="#" class="inline-block relative p-px">
                            <img src="https://assets.codepen.io/923404/finder.png" alt="Finder" class="block w-[75px] hover:scale-95" />
                        </a>
                        <a href="#" class="inline-block relative p-px">
                            <img src="https://assets.codepen.io/923404/map.png" alt="Maps" class="block w-[75px] hover:scale-95" />
                        </a>
                        <a href="#" class="inline-block relative p-px">
                            <img
                                src="https://assets.codepen.io/923404/messages.png"
                                alt="Messages"
                                class="block w-[75px] hover:scale-95"
                            />
                        </a>
                        <a href="#" class="inline-block relative p-px">
                            <img src="https://assets.codepen.io/923404/safari.png" alt="Safari" class="block w-[75px] hover:scale-95" />
                        </a>
                        <a href="#" class="inline-block relative p-px">
                            <img src="https://assets.codepen.io/923404/books.png" alt="Books" class="block w-[75px] hover:scale-95" />
                        </a>
                    </div>

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
                </div>
            </div>
        </div>
    }
}
