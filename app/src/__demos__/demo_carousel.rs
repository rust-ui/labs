use leptos::prelude::*;

#[component]
pub fn DemoCarousel() -> impl IntoView {
    view! {
        // * https://tailwindflex.com/@nikolai-petrovich/image-carousel-2
        <script src="/components/carousel.js" />

        <div class="relative">
            <div class="flex max-w-xl carousel">

                <div class="carousel-item">
                    <img
                        src="https://plus.unsplash.com/premium_photo-1664474619075-644dd191935f?q=80&w=1469&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                        alt="Carousel Image 1"
                        class="object-cover w-full h-96"
                    />
                </div>
                <div class="carousel-item">
                    <img
                        src="https://plus.unsplash.com/premium_photo-1664474619075-644dd191935f?q=80&w=1469&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                        alt="Carousel Image 2"
                        class="object-cover w-full h-96"
                    />
                </div>
                <div class="carousel-item">
                    <img
                        src="https://plus.unsplash.com/premium_photo-1664474619075-644dd191935f?q=80&w=1469&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                        alt="Carousel Image 3"
                        class="object-cover w-full h-96"
                    />
                </div>
            </div>

            // CAROUSEL CONTROLS
            <div class="flex absolute inset-y-0 left-0 justify-start items-center pl-4">
                <button class="p-2 text-white bg-gray-800 rounded-full hover:bg-gray-700 focus:outline-none carousel-control-prev">
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
            <div class="flex absolute inset-y-0 right-0 justify-end items-center pr-4">
                <button class="p-2 text-white bg-gray-800 rounded-full hover:bg-gray-700 focus:outline-none carousel-control-next">
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
