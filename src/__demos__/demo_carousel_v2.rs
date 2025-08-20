use leptos::prelude::*;

#[component]
pub fn DemoCarouselV2() -> impl IntoView {
    view! {
        <div class="relative mx-auto w-[600px]">
            <div class="relative slide">
                <img
                    class="object-cover w-full h-[300px]"
                    src="https://plus.unsplash.com/premium_photo-1664474619075-644dd191935f?q=80&w=1469&auto=format&fit=crop"
                />
                <div class="absolute bottom-0 py-3 px-5 w-full text-center text-white bg-black/40">
                    Slide 1
                </div>
            </div>
            <div class="relative slide">
                <img
                    class="object-cover w-full h-[300px]"
                    src="https://images.unsplash.com/photo-1584395630827-860eee694d7b?q=80&w=1469&auto=format&fit=crop"
                />
                <div class="absolute bottom-0 py-3 px-5 w-full text-center text-white bg-black/40">
                    Slide 2
                </div>
            </div>
            <div class="relative slide">
                <img
                    class="object-cover w-full h-[300px]"
                    src="https://images.unsplash.com/photo-1506744038136-46273834b3fb?q=80&w=1469&auto=format&fit=crop"
                />
                <div class="absolute bottom-0 py-3 px-5 w-full text-center text-white bg-black/40">
                    Slide 3
                </div>
            </div>
            <a
                class="absolute left-0 top-1/2 p-4 text-white -translate-y-1/2 cursor-pointer hover:text-amber-500 bg-black/30 hover:bg-black/50"
                onclick="moveSlide(-1)"
            >
                "❮"
            </a>
            <a
                class="absolute right-0 top-1/2 p-4 text-white -translate-y-1/2 cursor-pointer hover:text-amber-500 bg-black/30 hover:bg-black/50"
                onclick="moveSlide(1)"
            >
                "❯"
            </a>
        </div>
        <br />
        <div class="flex justify-center items-center space-x-5">
            <div class="w-4 h-4 rounded-full cursor-pointer dot" onclick="currentSlide(1)"></div>
            <div class="w-4 h-4 rounded-full cursor-pointer dot" onclick="currentSlide(2)"></div>
            <div class="w-4 h-4 rounded-full cursor-pointer dot" onclick="currentSlide(3)"></div>
        </div>

        <script src="/components/carousel_v2.js"></script>
    }
}
