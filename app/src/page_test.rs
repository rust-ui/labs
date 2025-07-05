use leptos::prelude::*;

#[component]
pub fn PageTest() -> impl IntoView {
    view! {
        <div class="p-8 min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900">
            <div class="mx-auto max-w-4xl">
                <h1 class="mb-12 text-4xl font-bold text-center text-transparent bg-clip-text bg-gradient-to-r from-pink-400 to-purple-400 md:text-5xl">
                    "Responsive Alternate Cards in CSS"
                </h1>

                <div class="p-6 bg-gray-700 rounded-3xl shadow-2xl md:p-8">
                    <div class="space-y-6">
                        // Card 1 - Normal layout
                        <div class="flex flex-col gap-4 p-4 bg-gray-200 rounded-2xl shadow-lg md:flex-row md:gap-6 md:p-6">
                            <div class="flex justify-center items-center w-full h-48 bg-gradient-to-br from-amber-400 to-orange-500 rounded-xl shadow-md md:w-1/3 md:h-32">
                                <div class="text-lg font-semibold text-white">"Nature Scene"</div>
                            </div>
                            <div class="flex-1 space-y-2">
                                <p class="text-sm leading-relaxed text-gray-700 md:text-base">
                                    "Nature's beauty encompasses a vast array of colors, sounds, and textures that evoke a sense of wonder. Its rhythms and patterns create a calming atmosphere that can rejuvenate the spirit."
                                </p>
                            </div>
                        </div>

                        // Card 2 - Reverse layout
                        <div class="flex flex-col gap-4 p-4 bg-gray-200 rounded-2xl shadow-lg md:flex-row-reverse md:gap-6 md:p-6">
                            <div class="flex justify-center items-center w-full h-48 bg-gradient-to-br from-teal-400 to-cyan-500 rounded-xl shadow-md md:w-1/3 md:h-32">
                                <div class="text-lg font-semibold text-white">"Wildlife"</div>
                            </div>
                            <div class="flex-1 space-y-2">
                                <p class="text-sm leading-relaxed text-gray-700 md:text-base">
                                    "The intricate balance of ecosystems showcases the interdependence of all living beings. Each element, from the smallest insect to the largest tree, plays a vital role in sustaining life."
                                </p>
                            </div>
                        </div>

                        // Card 3 - Normal layout
                        <div class="flex flex-col gap-4 p-4 bg-gray-200 rounded-2xl shadow-lg md:flex-row md:gap-6 md:p-6">
                            <div class="flex justify-center items-center w-full h-48 bg-gradient-to-br from-emerald-400 to-green-500 rounded-xl shadow-md md:w-1/3 md:h-32">
                                <div class="text-lg font-semibold text-white">"Landscape"</div>
                            </div>
                            <div class="flex-1 space-y-2">
                                <p class="text-sm leading-relaxed text-gray-700 md:text-base">
                                    "The ever-changing landscapes of nature remind us of the passage of time. Seasons bring transformations that create a dynamic environment filled with diverse flora and fauna."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
