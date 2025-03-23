use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

mod components {
    use super::*;
    clx! {Tabs, div, "w-full max-w-md [&_label]:cursor-pointer"}
    clx! {TabsList, div,
        "flex px-4 gap-[1px]",
        "peer-checked/tab-1:[&>label:nth-child(1)]:bg-blue-800",
        "peer-checked/tab-2:[&>label:nth-child(2)]:bg-green-800",
        "peer-checked/tab-3:[&>label:nth-child(3)]:bg-rose-800",
        "*:rounded-t-xl *:py-1 *:px-4 *:transition *:duration-300 hover:*:bg-zinc-800"
    }
    clx! {SubTabsList, div,
        "p-6 rounded-xl bg-zinc-800",
        "[&_label]:rounded-md [&_label]:text-white [&_label]:py-1 [&_label]:px-4 [&_label]:w-full [&_label]:transition [&_label]:duration-300 [&_label]:border-b [&_label]:border-zinc-500 peer-checked/tab-1:[&_#sub-tab-1]:opacity-100 peer-checked/tab-1:[&_#sub-tab-1]:pointer-events-auto peer-checked/tab-1:[&_#panels-contents-1]:opacity-100",
        "peer-checked/tab-2:[&_#sub-tab-2]:opacity-100 peer-checked/tab-2:[&_#sub-tab-2]:pointer-events-auto peer-checked/tab-2:[&_#panels-contents-2]:opacity-100",
        "peer-checked/tab-3:[&_#sub-tab-3]:opacity-100 peer-checked/tab-3:[&_#sub-tab-3]:pointer-events-auto peer-checked/tab-3:[&_#panels-contents-3]:opacity-100"
    }
    clx! {TabSubTriggers, div,
        "grid [template-grid-areas:'stack'] *:[grid-area:stack] *:transition-opacity *:duration-300 *:opacity-0 *:pointer-events-none *:flex *:gap-[1px] peer-checked/panel-1.1:[&_#sub-tab-1>label:nth-child(1)]:bg-blue-800 peer-checked/panel-1.2:[&_#sub-tab-1>label:nth-child(2)]:bg-blue-800 peer-checked/panel-1.3:[&_#sub-tab-1>label:nth-child(3)]:bg-blue-800 peer-checked/panel-2.1:[&_#sub-tab-2>label:nth-child(1)]:bg-green-800 peer-checked/panel-2.2:[&_#sub-tab-2>label:nth-child(2)]:bg-green-800 peer-checked/panel-2.3:[&_#sub-tab-2>label:nth-child(3)]:bg-green-800 peer-checked/panel-3.1:[&_#sub-tab-3>label:nth-child(1)]:bg-rose-800 peer-checked/panel-3.2:[&_#sub-tab-3>label:nth-child(2)]:bg-rose-800 peer-checked/panel-3.3:[&_#sub-tab-3>label:nth-child(3)]:bg-rose-800"
    }
}

pub use components::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn TabsTrigger(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] checked: bool,
    name: &'static str,
) -> impl IntoView {
    // TODO. Try to have a random id.
    let class = Memo::new(move |_| tw_merge!("sr-only", class()));

    view! { <input type="radio" name=name id=id class=class checked=checked /> }
}
