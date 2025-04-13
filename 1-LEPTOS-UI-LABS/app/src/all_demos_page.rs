use leptos::prelude::*;
use leptos_ui::{a, clx};

use crate::__demos__::_process_to_follow::ProcessToFollow;
use crate::__demos__::demo_cards_gradient::DemoCardsGradient;
use crate::__demos__::demo_carousel::DemoCarousel;
use crate::__demos__::demo_drawer::DemoDrawer;
use crate::shared::utils::params::QueryUtils;

const DEMO_1: &str = "Process to follow";
const DEMO_2: &str = "Carousel";
const DEMO_3: &str = "Drawer";
const DEMO_4: &str = "Gradient Card";
const DEMO_5: &str = "Slot_5";
const DEMO_6: &str = "Slot_6";
const DEMO_7: &str = "Slot_7";
const DEMO_8: &str = "Slot_8";

#[component]
pub fn AllDemosPage() -> impl IntoView {
    clx! {Sidenav, div, "flex flex-col h-full gap-4 bg-neutral-500 w-[300px]"}
    a! {SidenavLink, "px-4 py-2 hover:bg-neutral-600"}

    view! {
        <div class="flex gap-4 p-2 mx-4">
            <Sidenav>
                <SidenavLink href=format!("?demo={}", DEMO_1)>{DEMO_1}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_2)>{DEMO_2}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_3)>{DEMO_3}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_4)>{DEMO_4}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_5)>{DEMO_5}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_6)>{DEMO_6}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_7)>{DEMO_7}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_8)>{DEMO_8}</SidenavLink>
            </Sidenav>

            <RenderComponentFromQuery />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn RenderComponentFromQuery() -> impl IntoView {
    let demo_query = QueryUtils::extract_demo();

    view! {
        <div class="flex flex-col gap-4 w-full">
            <div class="w-full">
                {move || {
                    let demo = demo_query();
                    match demo.as_str() {
                        DEMO_1 => view! { <ProcessToFollow /> }.into_any(),
                        DEMO_2 => view! { <DemoCarousel /> }.into_any(),
                        DEMO_3 => view! { <DemoDrawer /> }.into_any(),
                        DEMO_4 => view! { <DemoCardsGradient /> }.into_any(),
                        _ => view! { <p>"Select a component to display"</p> }.into_any(),
                    }
                }}
            </div>
        </div>
    }
}
