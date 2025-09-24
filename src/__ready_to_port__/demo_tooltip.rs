use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

mod components {
    use super::*;
    clx! {Tooltip, div, "inline-block relative mx-0 whitespace-nowrap transition-all duration-300 ease-in-out group my-[5px]"}
}

pub use components::*;

use crate::components::ui::button::{Button, ButtonVariant};

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

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
    const SHARED_TRANSITION_CLASSES: &str = "absolute opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover:opacity-100 z-[1000000]";

    let tooltip_class = tw_merge!(
        SHARED_TRANSITION_CLASSES,
        "py-2 px-2.5 text-xs whitespace-nowrap shadow-lg text-background bg-foreground/90",
        class(),
        match data_position {
            TooltipPosition::Top => "left-1/2 bottom-full mb-1 -ml-2.5",
            TooltipPosition::Right => "bottom-1/2 left-full ml-2.5 -mb-3.5",
            TooltipPosition::Bottom => "left-1/2 top-full mt-1 -ml-2.5",
            TooltipPosition::Left => "bottom-1/2 right-full mr-2.5 -mb-3.5",
        },
    );

    let arrow_class = tw_merge!(
        SHARED_TRANSITION_CLASSES,
        "bg-transparent border-transparent border-6",
        match data_position {
            TooltipPosition::Top => "left-1/2 bottom-full -mb-2 border-t-foreground/90",
            TooltipPosition::Right => "bottom-1/2 left-full -mr-0.5 -mb-1 border-r-foreground/90",
            TooltipPosition::Bottom => "left-1/2 top-full -mt-2 border-b-foreground/90",
            TooltipPosition::Left => "bottom-1/2 right-full -mb-1 -ml-0.5 border-l-foreground/90",
        },
    );

    view! {
        <>
            <div data-name="TooltipArrow" class=arrow_class />
            <div data-name="TooltipContent" class=tooltip_class>
                {children()}
            </div>
        </>
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

        <div class="flex flex-col gap-4 mt-20 h-full text-center text-[18px]">
            <Tooltip>
                <span>Top</span>
                <TooltipContent data_position=TooltipPosition::Top>Tooltip on top</TooltipContent>
            </Tooltip>
            <Tooltip>
                <span>Right</span>
                <TooltipContent data_position=TooltipPosition::Right>
                    Tooltip on right
                </TooltipContent>
            </Tooltip>
            <Tooltip>
                <span>Bottom</span>
                <TooltipContent data_position=TooltipPosition::Bottom>
                    Tooltip bottom
                </TooltipContent>
            </Tooltip>
            <Tooltip>
                <span>Left</span>
                <TooltipContent data_position=TooltipPosition::Left>Tooltip left</TooltipContent>
            </Tooltip>

            <Tooltip>
                <Button variant=ButtonVariant::Secondary href="https://rust-ui.com">
                    Rust/UI
                </Button>
                <TooltipContent data_position=TooltipPosition::Right>
                    Tooltip on right
                </TooltipContent>
            </Tooltip>
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
