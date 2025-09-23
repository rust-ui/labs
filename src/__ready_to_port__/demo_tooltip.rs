use leptos::prelude::*;

#[component]
pub fn DemoTooltip() -> impl IntoView {
    view! {
        <style>
            {"
            /* JS adds this class to enable interaction */
            [data-name=\"TooltipContent\"].tooltip__interactive,
            [data-name=\"TooltipContent\"]:hover {
             pointer-events: auto;
            }
            "}
        </style>

        <div class="mt-20 h-full text-center text-[18px] text-[#444]">
            <div>
                <div
                    data-name="Tooltip"
                    data-position="Top"
                    class="inline-block relative mx-0 whitespace-nowrap transition-all duration-300 ease-in-out group my-[5px]"
                >
                    <span>Top</span>
                    <div
                        data-name="TooltipArrow"
                        class="absolute left-1/2 bottom-full -mb-6 bg-transparent border-transparent opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] border-6 border-t-black/80"
                    ></div>
                    <div
                        data-name="TooltipContent"
                        class="absolute left-1/2 bottom-full py-2 px-2.5 -mb-3 -ml-2.5 text-xs text-white whitespace-nowrap shadow-lg opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] bg-black/80"
                    >
                        Tooltip on top
                    </div>
                </div>
            </div>
            <div>
                <div
                    data-name="Tooltip"
                    data-position="Right"
                    class="inline-block relative mx-0 whitespace-nowrap transition-all duration-300 ease-in-out group my-[5px]"
                >
                    <span>Right</span>
                    <div
                        data-name="TooltipArrow"
                        class="absolute bottom-1/2 left-full -mr-0.5 -mb-1 bg-transparent border-transparent opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] border-6 border-r-black/80"
                    ></div>
                    <div
                        data-name="TooltipContent"
                        class="absolute bottom-1/2 left-full py-2 px-2.5 ml-2.5 -mb-3.5 text-xs text-white whitespace-nowrap shadow-lg opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] bg-black/80"
                    >
                        Tooltip on right
                    </div>
                </div>
            </div>
            <div>
                <div
                    data-name="Tooltip"
                    data-position="Bottom"
                    class="inline-block relative mx-0 whitespace-nowrap transition-all duration-300 ease-in-out group my-[5px]"
                >
                    <span>Bottom</span>
                    <div
                        data-name="TooltipArrow"
                        class="absolute left-1/2 top-full -mt-2 bg-transparent border-transparent opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] border-6 border-b-black/80"
                    ></div>
                    <div
                        data-name="TooltipContent"
                        class="absolute left-1/2 top-full py-2 px-2.5 mt-1 -ml-2.5 text-xs text-white whitespace-nowrap shadow-lg opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] bg-black/80"
                    >
                        Tooltip bottom
                    </div>
                </div>
            </div>
            <div>
                <div
                    data-name="Tooltip"
                    data-position="Left"
                    class="inline-block relative mx-0 whitespace-nowrap transition-all duration-300 ease-in-out group my-[5px]"
                >
                    <span>Left</span>
                    <div
                        data-name="TooltipArrow"
                        class="absolute bottom-1/2 right-full -mb-1 -ml-0.5 bg-transparent border-transparent opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] border-6 border-l-black/80"
                    ></div>
                    <div
                        data-name="TooltipContent"
                        class="absolute bottom-1/2 right-full py-2 px-2.5 mr-2.5 -mb-3.5 text-xs text-white whitespace-nowrap shadow-lg opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] bg-black/80"
                    >
                        Tooltip left
                    </div>
                </div>
            </div>

            <div
                data-name="Tooltip"
                data-position="Right"
                class="inline-block relative mx-0 whitespace-nowrap transition-all duration-300 ease-in-out group my-[5px]"
            >
                <a href="https://rust-ui.com">Rust/UI</a>
                <div
                    data-name="TooltipArrow"
                    class="absolute bottom-1/2 left-full -mr-0.5 -mb-1 bg-transparent border-transparent opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] border-6 border-r-black/80"
                ></div>
                <div
                    data-name="TooltipContent"
                    class="absolute bottom-1/2 left-full py-2 px-2.5 ml-2.5 -mb-3.5 text-xs text-white whitespace-nowrap shadow-lg opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] bg-black/80"
                >
                    Tooltip on right
                </div>
            </div>
        </div>

        <script>
            {"
            // Simplified JS - only handles pointer-events for text selection
            const tooltipTriggers = document.querySelectorAll('[data-name=\"Tooltip\"]');
            
            tooltipTriggers.forEach(trigger => {
             const content = trigger.querySelector('[data-name=\"TooltipContent\"]');
            
             trigger.addEventListener('mouseenter', () => {
               content.classList.add('tooltip__interactive');
             });
            
             trigger.addEventListener('mouseleave', () => {
               setTimeout(() => {
                 content.classList.remove('tooltip__interactive');
               }, 100);
             });
            });
            "}
        </script>
    }
}
