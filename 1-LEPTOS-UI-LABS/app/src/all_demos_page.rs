use leptos::prelude::*;

use crate::__demos__::{
    _process_to_follow::ProcessToFollow, demo_carousel::DemoCarousel, demo_drawer::DemoDrawer,
};

// * 1️⃣ ---------- Put the name of your demo here 👇 ----------
const DEMO_1: &str = "--- Process to Follow ---";
const DEMO_2: &str = "Carousel";
const DEMO_3: &str = "Drawer";

// * 2️⃣ ---------- Put the name of your demo here 👆 ----------

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn AllDemosPage() -> impl IntoView {
    let demo = RwSignal::new(DEMO_1.to_string());

    // *  ------------------ 2️⃣ Put your Component here 👇 ------------------
    let render_demo = move || match demo.get().as_str() {
        DEMO_1 => view! { <ProcessToFollow /> }.into_any(),
        DEMO_2 => view! { <DemoCarousel /> }.into_any(),
        DEMO_3 => view! { <DemoDrawer /> }.into_any(),
        _ => view! {}.into_any(),
    };
    // *  ------------------ 2️⃣ Put your Component here 👆 ------------------

    view! {
        <div class="flex gap-4 p-2 mx-4">
            <div class="flex flex-col h-full gap-4 bg-neutral-500 w-[300px]">
                // * 3️⃣ ---------- Put the name of your demo here 👇 ----------
                <button on:click=move |_| { demo.set(DEMO_1.into()) }>{DEMO_1}</button>
                <button on:click=move |_| { demo.set(DEMO_2.into()) }>{DEMO_2}</button>
                <button on:click=move |_| { demo.set(DEMO_3.into()) }>{DEMO_3}</button>

            // * 3️⃣ ---------- Put the name of your demo here 👆 ----------
            </div>

            <div class="w-full">{render_demo}</div>
        </div>
    }
}
