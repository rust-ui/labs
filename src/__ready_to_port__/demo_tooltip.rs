use leptos::prelude::*;
use tw_merge::tw_merge;

#[derive(Default)]
pub enum TooltipPosition {
    #[default]
    Top,
    Left,
    Right,
    Bottom,
}

#[component]
pub fn TooltipContent(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(default = TooltipPosition::default())] data_position: TooltipPosition,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "absolute py-2 px-2.5 text-xs whitespace-nowrap shadow-lg opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 text-background z-[1000000] bg-foreground/90",
        class(),
        match data_position {
            TooltipPosition::Top => "left-1/2 bottom-full -mb-3 -ml-2.5",
            TooltipPosition::Right => "bottom-1/2 left-full ml-2.5 -mb-3.5",
            TooltipPosition::Bottom => "left-1/2 top-full mt-1 -ml-2.5",
            TooltipPosition::Left => "bottom-1/2 right-full mr-2.5 -mb-3.5",
        },
    );

    view! {
        <div data-name="TooltipContent" class=merged_class>
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

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

        <div class="mt-20 h-full text-center text-[18px]">
            <div>
                <div
                    data-name="Tooltip"
                    data-position="Top"
                    class="inline-block relative mx-0 whitespace-nowrap transition-all duration-300 ease-in-out group my-[5px]"
                >
                    <span>Top</span>
                    <div
                        data-name="TooltipArrow"
                        class="absolute left-1/2 bottom-full -mb-6 bg-transparent border-transparent opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] border-6 border-t-foreground/90"
                    ></div>
                    <TooltipContent data_position=TooltipPosition::Top>
                        Tooltip on top
                    </TooltipContent>
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
                        class="absolute bottom-1/2 left-full -mr-0.5 -mb-1 bg-transparent border-transparent opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] border-6 border-r-foreground/90"
                    ></div>
                    <TooltipContent data_position=TooltipPosition::Right>
                        Tooltip on right
                    </TooltipContent>
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
                        class="absolute left-1/2 top-full -mt-2 bg-transparent border-transparent opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] border-6 border-b-foreground/90"
                    ></div>
                    <TooltipContent data_position=TooltipPosition::Bottom>
                        Tooltip bottom
                    </TooltipContent>
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
                        class="absolute bottom-1/2 right-full -mb-1 -ml-0.5 bg-transparent border-transparent opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] border-6 border-l-foreground/90"
                    ></div>
                    <TooltipContent data_position=TooltipPosition::Left>
                        Tooltip left
                    </TooltipContent>
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
                    class="absolute bottom-1/2 left-full -mr-0.5 -mb-1 bg-transparent border-transparent opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000] border-6 border-r-foreground/90"
                ></div>
                <TooltipContent data_position=TooltipPosition::Right>
                    Tooltip on right
                </TooltipContent>
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
