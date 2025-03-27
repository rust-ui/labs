use leptos::prelude::*;
use leptos_meta::Stylesheet;
use tw_merge::*;

#[component]
pub fn DemoJsTabsAnimated() -> impl IntoView {
    view! {
        <Stylesheet id="tabs-animated" href="/components/tabs_animated.css" />

        <Tabs>
            <TabButton class="active" transition_index=1>
                Home
            </TabButton>
            <TabButton transition_index=2>Interactions</TabButton>
            <TabButton transition_index=3>Resources</TabButton>
            <TabButton transition_index=4>Docs</TabButton>
        </Tabs>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn Tabs(#[prop(into, optional)] class: Signal<String>, children: Children) -> impl IntoView {
    let class = Memo::new(move |_| {
        tw_merge!(
            "flex gap-2 p-1 w-fit bg-zinc-100 border border-zinc-300 rounded-[24px] shadow-md",
            class()
        )
    });

    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn TabButton(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into)] transition_index: i32,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        tw_merge!(
            "tab",
            "p-[12px_16px] border-none bg-transparent rounded-[20px] text-base text-[#71717a] cursor-pointer relative font-medium h-fit", 
            class()
        )
    });

    let formatted_transition_name =
        move || format!("view-transition-name: tab-{}", transition_index);

    view! {
        <button
            class=class
            style=formatted_transition_name
            onclick="document.startViewTransition(() => {
            document.querySelector('.tab.active').classList.remove('active');
            this.classList.add('active');
            })"
        >
            {children()}
        </button>
    }
}
