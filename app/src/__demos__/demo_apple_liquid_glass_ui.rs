use leptos::prelude::*;
use leptos_meta::Stylesheet;

// Aknowledgement: https://codepen.io/joshua_k_y/pen/zYzQZqy

#[component]
pub fn DemoAppleLiquidGlassUi() -> impl IntoView {
    view! {
        <Stylesheet href="/components/apple_liquid_glass_ui.css" />

        <div class="mainDiv">
            <div class="container container__inline">
                <div class="glass-container glass_container__rounded glass_container__large">
                    <div class="glass__filter"></div>
                    <div class="glass__overlay"></div>
                    <div class="glass__specular"></div>
                    <div class="glass__content glass__content__inline">
                        <div class="player">
                            <div class="player__thumb">
                                <img
                                    class="player__img"
                                    src="https://images.unsplash.com/photo-1619983081593-e2ba5b543168?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3wzMjM4NDZ8MHwxfHJhbmRvbXx8fHx8fHx8fDE3NDk1NzAwNDV8&ixlib=rb-4.1.0&q=80&w=400"
                                    alt="All Of Me"
                                />
                                <div class="player__legend">
                                    <h3 class="player__legend__title">All Of Me</h3>
                                    <span class="player__legend__sub-title">Nao</span>
                                </div>
                            </div>
                            <div class="player__controls">
                                <div class="player__controls__play">
                                    <svg viewBox="0 0 448 512" width="24" title="play">
                                        <path
                                            fill="black"
                                            d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z"
                                        />
                                    </svg>
                                </div>
                                <div class="player__controls__ff">
                                    <svg viewBox="0 0 448 512" width="24" title="play">
                                        <path
                                            fill="black"
                                            d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z"
                                        />
                                    </svg>
                                    <svg viewBox="0 0 448 512" width="24" title="play">
                                        <path
                                            fill="black"
                                            d="M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z"
                                        />
                                    </svg>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="container container__inline">
                <div class="glass-container glass_container__rounded">
                    <div class="glass__filter"></div>
                    <div class="glass__overlay"></div>
                    <div class="glass__specular"></div>
                    <div class="glass__content glass__content__alone">
                        <div class="glass-item">
                            <svg viewBox="0 0 512 512" width="40" title="search">
                                <path d="M505 442.7L405.3 343c-4.5-4.5-10.6-7-17-7H372c27.6-35.3 44-79.7 44-128C416 93.1 322.9 0 208 0S0 93.1 0 208s93.1 208 208 208c48.3 0 92.7-16.4 128-44v16.3c0 6.4 2.5 12.5 7 17l99.7 99.7c9.4 9.4 24.6 9.4 33.9 0l28.3-28.3c9.4-9.4 9.4-24.6.1-34zM208 336c-70.7 0-128-57.2-128-128 0-70.7 57.2-128 128-128 70.7 0 128 57.2 128 128 0 70.7-57.2 128-128 128z" />
                            </svg>
                        </div>
                    </div>
                </div>
            </div>

            <div class="container">
                <div class="glass-container">
                    <div class="glass__filter"></div>
                    <div class="glass__overlay"></div>
                    <div class="glass__specular"></div>
                    <div class="glass__content">
                        <a href="#">
                            <img src="https://assets.codepen.io/923404/finder.png" alt="Finder" />
                        </a>
                        <a href="#">
                            <img src="https://assets.codepen.io/923404/map.png" alt="Maps" />
                        </a>
                        <a href="#">
                            <img
                                src="https://assets.codepen.io/923404/messages.png"
                                alt="Messages"
                            />
                        </a>
                        <a href="#">
                            <img src="https://assets.codepen.io/923404/safari.png" alt="Safari" />
                        </a>
                        <a href="#">
                            <img src="https://assets.codepen.io/923404/books.png" alt="Books" />
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
