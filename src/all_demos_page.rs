use leptos::prelude::*;
use leptos_ui::clx;

use crate::__demos__::_card_component_port_to_rust_ui::Card__ComponentsPortToRustUI;
use crate::all_demos::{DemoItem, ALL_DEMOS};
use crate::shared::utils::query::{QueryUtils, QUERY};

#[component]
pub fn AllDemosPage() -> impl IntoView {
    clx! {Sidenav, div, "flex flex-col h-full gap-1 bg-neutral-500 w-[300px]"}
    clx! {SidenavLink, a, "px-4 py-2 hover:bg-neutral-600"}

    let all_demos: Vec<&str> = ALL_DEMOS.iter().map(|demo| demo.name).collect();

    view! {
        <div class="flex gap-4 p-2 mx-4">
            <Sidenav>
                {all_demos
                    .into_iter()
                    .map(|demo| {
                        view! {
                            // href=format!("?demo={}", demo). // Force the reload of the page
                            <SidenavLink onclick=format!(
                                "window.location.href='?demo={}'; return false;",
                                demo,
                            )>{demo}</SidenavLink>
                        }
                    })
                    .collect_view()}
            </Sidenav>

            <RenderDemoFromQuery demos=ALL_DEMOS.to_vec() />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn RenderDemoFromQuery(demos: Vec<DemoItem>) -> impl IntoView {
    let demo_query = QueryUtils::extract(QUERY::DEMO.to_string());

    view! {
        <div class="flex flex-col gap-4 w-full">
            <Card__ComponentsPortToRustUI />

            <div class="w-full">
                {move || {
                    let current_demo = demo_query();
                    if let Some(demo) = demos.iter().find(|d| d.name == current_demo) {
                        (demo.render_fn)()
                    } else {

                        view! {
                            <p class="text-2xl font-bold text-center text-orange-500">
                                "<---- Select a component to display from the Sidenav 😄"
                            </p>
                        }
                            .into_any()
                    }
                }}
            </div>
        </div>
    }
}
