use leptos::prelude::*;

#[component]
pub fn DemoMobileStack() -> impl IntoView {
    view! {
        <style>
            {r#"
            ::view-transition-group(item-0) { animation-delay: 0s; }
            ::view-transition-group(item-1) { animation-delay: 0.01s; }
            ::view-transition-group(item-2) { animation-delay: 0.02s; }
            ::view-transition-group(item-3) { animation-delay: 0.03s; }
            ::view-transition-group(item-4) { animation-delay: 0.04s; }
            ::view-transition-group(item-5) { animation-delay: 0.05s; }
            ::view-transition-group(item-6) { animation-delay: 0.06s; }
            ::view-transition-group(item-7) { animation-delay: 0.07s; }
            ::view-transition-group(item-8) { animation-delay: 0.08s; }
            ::view-transition-group(item-9) { animation-delay: 0.09s; }
            
            #grid {
               inline-size: min(100vw, 300px);
               min-block-size: 677px;
               box-shadow: inset 0 -1px 0 black;
            }
            #grid > div { grid-area: 1/1/2/2; }
            #grid.open > div { grid-area: initial; }
            "#}
        </style>

        <div class="bg-[radial-gradient(circle,#555,#111)]">

            <div class="grid place-content-center m-0 h-screen">

                <div
                    id="grid"
                    class="grid gap-4 p-4 mx-auto border-t rounded-[40px] overflow-clip bg-[#221b1b] border-[#555]"
                >
                    <div
                        class="grid place-items-center text-2xl text-white bg-orange-500 transition-colors duration-150 first:z-10 hover:text-black hover:bg-white rounded-[50px] w-[50px] aspect-square"
                        style="view-transition-name: item-0; --sibling-index: 0;"
                    >
                        {"☰"}
                    </div>
                    <div
                        class="grid place-items-center text-2xl text-white bg-orange-500 transition-colors duration-150 hover:text-black hover:bg-white rounded-[50px] w-[50px] aspect-square"
                        style="view-transition-name: item-1; --sibling-index: 1;"
                    >
                        {"👍"}
                    </div>
                    <div
                        class="grid place-items-center text-2xl text-white bg-orange-500 transition-colors duration-150 hover:text-black hover:bg-white rounded-[50px] w-[50px] aspect-square"
                        style="view-transition-name: item-2; --sibling-index: 2;"
                    >
                        {"👎"}
                    </div>
                    <div
                        class="grid place-items-center text-2xl text-white bg-orange-500 transition-colors duration-150 hover:text-black hover:bg-white rounded-[50px] w-[50px] aspect-square"
                        style="view-transition-name: item-3; --sibling-index: 3;"
                    >
                        {"🎸"}
                    </div>
                    <div
                        class="grid place-items-center text-2xl text-white bg-orange-500 transition-colors duration-150 hover:text-black hover:bg-white rounded-[50px] w-[50px] aspect-square"
                        style="view-transition-name: item-4; --sibling-index: 4;"
                    >
                        {"🍽️"}
                    </div>
                    <div
                        class="grid place-items-center text-2xl text-white bg-orange-500 transition-colors duration-150 hover:text-black hover:bg-white rounded-[50px] w-[50px] aspect-square"
                        style="view-transition-name: item-5; --sibling-index: 5;"
                    >
                        {"🍈"}
                    </div>
                    <div
                        class="grid place-items-center text-2xl text-white bg-orange-500 transition-colors duration-150 hover:text-black hover:bg-white rounded-[50px] w-[50px] aspect-square"
                        style="view-transition-name: item-6; --sibling-index: 6;"
                    >
                        {"🥓"}
                    </div>
                    <div
                        class="grid place-items-center text-2xl text-white bg-orange-500 transition-colors duration-150 hover:text-black hover:bg-white rounded-[50px] w-[50px] aspect-square"
                        style="view-transition-name: item-7; --sibling-index: 7;"
                    >
                        {"🪜"}
                    </div>
                    <div
                        class="grid place-items-center text-2xl text-white bg-orange-500 transition-colors duration-150 hover:text-black hover:bg-white rounded-[50px] w-[50px] aspect-square"
                        style="view-transition-name: item-8; --sibling-index: 8;"
                    >
                        {"🎥"}
                    </div>
                    <div
                        class="grid place-items-center text-2xl text-white bg-orange-500 transition-colors duration-150 hover:text-black hover:bg-white rounded-[50px] w-[50px] aspect-square"
                        style="view-transition-name: item-9; --sibling-index: 9;"
                    >
                        {"🐟"}
                    </div>
                </div>

            </div>

        </div>

        <script src="/components/mobile_stack.js"></script>
    }
}
