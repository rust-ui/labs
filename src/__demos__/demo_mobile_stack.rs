use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoMobileStack() -> impl IntoView {
    view! {
        <Stylesheet href="/components/mobile_stack.css" />

        <div class="bg-[radial-gradient(circle,#555,#111)]">

            <div class="grid place-content-center m-0 h-screen">

                <div id="grid" class="p-4 mx-auto rounded-[40px] overflow-clip grid bg-[#221b1b] gap-4 border-t border-[#555]">
                    <div class="text-2xl rounded-[50px] grid place-items-center w-[50px] text-white hover:text-black aspect-square transition-colors duration-150 first:z-10 hover:bg-white bg-orange-500" style="view-transition-name: item-0; --sibling-index: 0;">{"☰"}</div>
                    <div class="text-2xl rounded-[50px] grid place-items-center w-[50px] text-white hover:text-black aspect-square transition-colors duration-150 hover:bg-white bg-orange-500" style="view-transition-name: item-1; --sibling-index: 1;">{"👍"}</div>
                    <div class="text-2xl rounded-[50px] grid place-items-center w-[50px] text-white hover:text-black aspect-square transition-colors duration-150 hover:bg-white bg-orange-500" style="view-transition-name: item-2; --sibling-index: 2;">{"👎"}</div>
                    <div class="text-2xl rounded-[50px] grid place-items-center w-[50px] text-white hover:text-black aspect-square transition-colors duration-150 hover:bg-white bg-orange-500" style="view-transition-name: item-3; --sibling-index: 3;">{"🎸"}</div>
                    <div class="text-2xl rounded-[50px] grid place-items-center w-[50px] text-white hover:text-black aspect-square transition-colors duration-150 hover:bg-white bg-orange-500" style="view-transition-name: item-4; --sibling-index: 4;">{"🍽️"}</div>
                    <div class="text-2xl rounded-[50px] grid place-items-center w-[50px] text-white hover:text-black aspect-square transition-colors duration-150 hover:bg-white bg-orange-500" style="view-transition-name: item-5; --sibling-index: 5;">{"🍈"}</div>
                    <div class="text-2xl rounded-[50px] grid place-items-center w-[50px] text-white hover:text-black aspect-square transition-colors duration-150 hover:bg-white bg-orange-500" style="view-transition-name: item-6; --sibling-index: 6;">{"🥓"}</div>
                    <div class="text-2xl rounded-[50px] grid place-items-center w-[50px] text-white hover:text-black aspect-square transition-colors duration-150 hover:bg-white bg-orange-500" style="view-transition-name: item-7; --sibling-index: 7;">{"🪜"}</div>
                    <div class="text-2xl rounded-[50px] grid place-items-center w-[50px] text-white hover:text-black aspect-square transition-colors duration-150 hover:bg-white bg-orange-500" style="view-transition-name: item-8; --sibling-index: 8;">{"🎥"}</div>
                    <div class="text-2xl rounded-[50px] grid place-items-center w-[50px] text-white hover:text-black aspect-square transition-colors duration-150 hover:bg-white bg-orange-500" style="view-transition-name: item-9; --sibling-index: 9;">{"🐟"}</div>
                </div>

            </div>

        </div>

        <script src="/components/mobile_stack.js"></script>
    }
}
