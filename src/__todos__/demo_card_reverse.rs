use leptos::prelude::*;

use crate::components::ui::card::{Card, CardContent, CardDescription, CardTitle};

#[component]
pub fn DemoCardReverse() -> impl IntoView {
    view! {
        <div class="p-8 min-h-screen">
            <div class="mx-auto max-w-4xl">
                <h1 class="mb-12 text-4xl font-bold text-center text-gray-900 md:text-5xl">
                    "Responsive Alternate Cards"
                </h1>

                <div class="space-y-6">
                    // Card 1 - Normal layout
                    <Card class="flex flex-col gap-6 md:flex-row">
                        <div class="flex justify-center items-center w-full h-48 bg-gray-200 rounded-lg md:w-1/3 md:h-32 font-medium text-gray-600">
                            "Image Placeholder"
                        </div>
                        <CardContent class="flex-1 pt-0">
                            <CardTitle class="mb-3">"Nature's Beauty"</CardTitle>
                            <CardDescription>
                                "Nature's beauty encompasses a vast array of colors, sounds, and textures that evoke a sense of wonder. Its rhythms and patterns create a calming atmosphere that can rejuvenate the spirit."
                            </CardDescription>
                        </CardContent>
                    </Card>

                    // Card 2 - Reverse layout
                    <Card class="flex flex-col gap-6 md:flex-row-reverse">
                        <div class="flex justify-center items-center w-full h-48 bg-gray-200 rounded-lg md:w-1/3 md:h-32 font-medium text-gray-600">
                            "Image Placeholder"
                        </div>
                        <CardContent class="flex-1 pt-0">
                            <CardTitle class="mb-3">"Ecosystem Balance"</CardTitle>
                            <CardDescription>
                                "The intricate balance of ecosystems showcases the interdependence of all living beings. Each element, from the smallest insect to the largest tree, plays a vital role in sustaining life."
                            </CardDescription>
                        </CardContent>
                    </Card>

                    // Card 3 - Normal layout
                    <Card class="flex flex-col gap-6 md:flex-row">
                        <div class="flex justify-center items-center w-full h-48 bg-gray-200 rounded-lg md:w-1/3 md:h-32 font-medium text-gray-600">
                            "Image Placeholder"
                        </div>
                        <CardContent class="flex-1 pt-0">
                            <CardTitle class="mb-3">"Changing Landscapes"</CardTitle>
                            <CardDescription>
                                "The ever-changing landscapes of nature remind us of the passage of time. Seasons bring transformations that create a dynamic environment filled with diverse flora and fauna."
                            </CardDescription>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </div>
    }
}
