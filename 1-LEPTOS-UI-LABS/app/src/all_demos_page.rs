use leptos::prelude::*;
use leptos_ui::{a, clx};

use crate::__demos__::_process_to_follow::ProcessToFollow;
use crate::__demos__::demo_cards_gradient::DemoCardsGradient;
use crate::__demos__::demo_carousel::DemoCarousel;
use crate::__demos__::demo_drawer::DemoDrawer;
use crate::__demos__::demo_dynamic_custom_cursor::DemoCustomCursor;
use crate::__demos__::demo_word_rotation::DemoWordRotation;
use crate::shared::utils::params::QueryUtils;
// * 1️⃣ Add the name of your demo here
const DEMO_1: &str = "Process to follow";
const DEMO_2: &str = "Carousel";
const DEMO_3: &str = "Drawer";
const DEMO_4: &str = "Gradient Card";
const DEMO_5: &str = "Word Rotation";
const DEMO_6: &str = "Grasp Custom Cursor";
const DEMO_7: &str = "Slot_7";
const DEMO_8: &str = "Slot_8";
const DEMO_9: &str = "Slot_9";
const DEMO_10: &str = "Slot_10";
const DEMO_11: &str = "Slot_11";
const DEMO_12: &str = "Slot_12";
const DEMO_13: &str = "Slot_13";
const DEMO_14: &str = "Slot_14";
const DEMO_15: &str = "Slot_15";

#[component]
pub fn AllDemosPage() -> impl IntoView {
    clx! {Sidenav, div, "flex flex-col h-full gap-1 bg-neutral-500 w-[300px]"}
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
                <SidenavLink href=format!("?demo={}", DEMO_9)>{DEMO_9}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_10)>{DEMO_10}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_11)>{DEMO_11}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_12)>{DEMO_12}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_13)>{DEMO_13}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_14)>{DEMO_14}</SidenavLink>
                <SidenavLink href=format!("?demo={}", DEMO_15)>{DEMO_15}</SidenavLink>
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
                // * 2️⃣ Add your demo Component here

                {move || {
                    let demo = demo_query();
                    match demo.as_str() {
                        DEMO_1 => view! { <ProcessToFollow /> }.into_any(),
                        DEMO_2 => view! { <DemoCarousel /> }.into_any(),
                        DEMO_3 => view! { <DemoDrawer /> }.into_any(),
                        DEMO_4 => view! { <DemoCardsGradient /> }.into_any(),
                        DEMO_5 => view! { <DemoWordRotation /> }.into_any(),
                        DEMO_6 => view! { <DemoCustomCursor /> }.into_any(),
                        DEMO_7 => view! { "SLOT_7" }.into_any(),
                        DEMO_8 => view! { "SLOT_8" }.into_any(),
                        DEMO_9 => view! { "SLOT_9" }.into_any(),
                        DEMO_10 => view! { "SLOT_10" }.into_any(),
                        DEMO_11 => view! { "SLOT_11" }.into_any(),
                        DEMO_12 => view! { "SLOT_12" }.into_any(),
                        DEMO_13 => view! { "SLOT_13" }.into_any(),
                        DEMO_14 => view! { "SLOT_14" }.into_any(),
                        DEMO_15 => view! { "SLOT_15" }.into_any(),
                        _ => view! { <p>"Select a component to display"</p> }.into_any(),
                    }
                }}
            </div>
        </div>
    }
}
