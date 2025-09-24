use icons::{ChevronDown, ChevronRight};
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

mod components {
    use super::*;
    clx! {Accordion, div, "divide-y divide-input w-full"}
    clx! {AccordionTitle, h4, "text-sm font-medium"}
    clx! {TriggerRoot, summary, "flex justify-between items-center py-4 list-none cursor-pointer [&_svg:not([class*='size-'])]:size-4"}
    clx! {AccordionHeader, div, "flex gap-2 items-center [&_svg:not([class*='size-'])]:size-4"}
    clx! {AccordionContent, article, "pb-4"}
    clx! {AccordionDescription, p, "text-muted-foreground"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Default)]
pub enum AccordionTriggerIcon {
    #[default]
    ChevronDown,
    Plus,
}

#[component]
pub fn AccordionTrigger(
    #[prop(into, optional)] class: Signal<String>,
    // TODO. AccrodionTriggerIcon
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| tw_merge!("", class()));

    view! {
        <TriggerRoot class=class>
            {children()} <ChevronDown class="block transition-all duration-300 group-open:rotate-180" />
        </TriggerRoot>
    }
}

// TODO.
#[component]
pub fn AccordionTriggerSidenav(children: Children, #[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    let class = Memo::new(move |_| {
        tw_merge!(
            "w-full flex gap-2 justify-between items-center p-2 font-medium hover:cursor-pointer marker:content-none [&_svg:not([class*='size-'])]:size-4  h-8 text-sm",
            class()
        )
    });

    view! {
        <summary data-name="AccordionTriggerSidenav" class=class>
            {children()}
            <ChevronRight class="transition group-open:rotate-90" />
        </summary>
    }
}

#[component]
pub fn AccordionItem(
    children: Children,
    #[prop(into, optional)] open: bool,
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let class = Memo::new(move |_| tw_merge!("group w-full", class()));

    view! {
        <details class=class open=open>
            {children()}
        </details>
    }
}
