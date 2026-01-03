use leptos::prelude::*;
use leptos_ui::clx;

use crate::__TODOS__::all_demos::{ALL_DEMOS, DemoItem};
use crate::utils::query::{QUERY, QueryUtils};

#[component]
pub fn AllDemosPage() -> impl IntoView {
    clx! {Sidenav, aside, "hidden md:flex flex-col h-full gap-1 w-[220px] shrink-0 border-r border-border bg-muted/40 rounded-lg"}

    let all_demos: Vec<&str> = ALL_DEMOS.iter().map(|demo| demo.name).collect();

    view! {
        <div class="flex gap-6 p-4">
            <Sidenav>
                <div class="py-3 px-4 text-xs font-semibold tracking-wider uppercase text-muted-foreground">
                    "Components"
                </div>
                {all_demos
                    .into_iter()
                    .map(|demo| {
                        view! {
                            <a
                                class="py-2 px-4 mx-2 text-sm rounded-md transition-colors cursor-pointer text-muted-foreground hover:text-foreground hover:bg-accent"
                                onclick=format!(
                                    "window.location.href='?demo={}'; return false;",
                                    demo,
                                )
                            >
                                {demo}
                            </a>
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

    // "relative" to make sure there is no issue in the DemoComponents.
    clx! {Preview, div, "relative w-full min-h-[400px] border border-border rounded-lg flex items-center justify-center bg-background"}

    view! {
        <div class="flex flex-col flex-1 gap-4">
            // Mobile-only dropdown
            <select
                class="py-2 px-4 w-full rounded-lg border md:hidden border-border bg-background text-foreground"
                onchange="window.location.href='?demo=' + this.value"
            >
                <option value="" disabled selected>
                    "Select a component"
                </option>
                {demos
                    .iter()
                    .map(|demo| {
                        view! { <option value=demo.name>{demo.name}</option> }
                    })
                    .collect_view()}
            </select>

            <Preview>
                {move || {
                    let current_demo = demo_query();
                    if let Some(demo) = demos.iter().find(|d| d.name == current_demo) {
                        (demo.render_fn)()
                    } else {
                        view! {
                            <p class="text-lg text-center text-muted-foreground">
                                "Select a component"
                            </p>
                        }
                            .into_any()
                    }
                }}
            </Preview>
        </div>
    }
}
