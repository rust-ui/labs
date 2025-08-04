use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

// TODO. The Autoplay effect should be optional.
// TODO. If Autoplay is enabled, display an elapsed timer.

const TIMER_INTERVAL: i32 = 4000;

#[component]
pub fn DemoCarousel() -> impl IntoView {
    let images = RwSignal::new(vec![
        "https://plus.unsplash.com/premium_photo-1664474619075-644dd191935f?q=80&w=1469&auto=format&fit=crop",
        "https://images.unsplash.com/photo-1584395630827-860eee694d7b?q=80&w=1469&auto=format&fit=crop",
        "https://images.unsplash.com/photo-1506744038136-46273834b3fb?q=80&w=1469&auto=format&fit=crop",
    ]);

    let index = RwSignal::new(0);

    let next = {
        move |_| {
            let len = images.get().len();
            index.update(|i| *i = (*i + 1) % len);
        }
    };

    let prev = {
        move |_| {
            let len = images.get().len();
            index.update(|i| {
                if *i == 0 {
                    *i = len - 1;
                } else {
                    *i -= 1;
                }
            });
        }
    };

    // Autoplay effect: every 4s update index
    Effect::new({
        move |_| {
            let window = window();
            let closure = Closure::wrap(Box::new(move || {
                let len = images.get().len();
                index.update(|i| *i = (*i + 1) % len);
            }) as Box<dyn FnMut()>);

            // Set interval every 4 seconds
            window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    TIMER_INTERVAL,
                )
                .expect("interval should be set");

            closure.forget(); // prevent dropping
        }
    });

    view! {
        <div class="relative mx-auto max-w-xl">
            <div class="overflow-hidden h-96 rounded-lg">
                <img
                    class="object-cover w-full h-full transition-all duration-700 ease-in-out"
                    src=move || images.get()[index.get()]
                    alt="Carousel image"
                />
            </div>

            // Prev button
            <div class="flex absolute inset-y-0 left-0 items-center pl-4">
                <button
                    on:click=prev
                    class="p-2 text-white bg-gray-800 rounded-full hover:bg-gray-700 focus:outline-none"
                >
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M15 19l-7-7 7-7"
                        />
                    </svg>
                </button>
            </div>

            // Next button
            <div class="flex absolute inset-y-0 right-0 items-center pr-4">
                <button
                    on:click=next
                    class="p-2 text-white bg-gray-800 rounded-full hover:bg-gray-700 focus:outline-none"
                >
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M9 5l7 7-7 7"
                        />
                    </svg>
                </button>
            </div>
        </div>
    }
}
