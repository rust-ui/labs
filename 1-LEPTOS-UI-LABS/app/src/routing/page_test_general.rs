use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::clx;
use tw_merge::*;

#[component]
pub fn PageTestGeneral() -> impl IntoView {
    view! {
        <div>
            <h1>Page Test General</h1>
            <DemoCarousel />
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
