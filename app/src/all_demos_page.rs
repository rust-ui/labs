use leptos::prelude::*;
use leptos_ui::{a, clx};

use crate::all_demos::{ALL_DEMOS, DemoItem};
use crate::shared::utils::query::QueryUtils;

#[component]
pub fn AllDemosPage() -> impl IntoView {
    clx! {Sidenav, div, "flex flex-col h-full gap-1 bg-neutral-500 w-[300px]"}
    a! {SidenavLink, "px-4 py-2 hover:bg-neutral-600"}

    let all_demos: Vec<&str> = ALL_DEMOS.iter().map(|demo| demo.name).collect();

    view! {
        <div class="flex gap-4 p-2 mx-4">
            <Sidenav>
                {all_demos.into_iter().map(|demo| {
                    view! {
                        <SidenavLink href=format!("?demo={}", demo)>{demo}</SidenavLink>
                    }
                }).collect_view()}
            </Sidenav>

            <RenderComponentFromQuery demos=ALL_DEMOS.to_vec() />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn RenderComponentFromQuery(demos: Vec<DemoItem>) -> impl IntoView {
    let demo_query = QueryUtils::extract_demo();

    view! {
        <div class="flex flex-col w-full gap-4">
            <div class="w-full">
                {move || {
                    let current_demo = demo_query();

                    if let Some(demo) = demos.iter().find(|d| d.name == current_demo) {
                        (demo.render_fn)()
                    } else {
                        view! { <p>"Select a component to display"</p> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
