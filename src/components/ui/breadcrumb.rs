use icons::{ChevronRight, Ellipsis};
use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Breadcrumb, nav, ""}
    clx! {BreadcrumbList, ol, "flex flex-wrap gap-1 items-center text-sm break-words sm:gap-2 text-muted-foreground"}
    clx! {BreadcrumbItem, li, "inline-flex gap-1 items-center [&_svg:not([class*='size-'])]:size-4"}
    clx! {BreadcrumbLink, a, "transition-colors hover:text-foreground"}
    clx! {RootSeparator, li, "[&>svg]:size-3.5 [&_svg:not([class*='size-'])]:size-4"}
    clx! {RootPage, span, "font-normal text-foreground"}
    clx! {RootEllipsisBtn, button, "flex items-center gap-1"}
    clx! {RootEllipsis, span, "flex items-center justify-center size-4"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn BreadcrumbSeparator(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <RootSeparator class=class attr:role="presentation" attr:aria-hidden="true">
            <ChevronRight />
        </RootSeparator>
    }
}

#[component]
pub fn BreadcrumbPage(#[prop(into, optional)] class: Signal<String>, children: Children) -> impl IntoView {
    // TODO. aria_disabled
    view! {
        <RootPage class=class attr:role="link" attr:aria-disabled="true" attr:aria-current="page">
            {children()}
        </RootPage>
    }
}

#[component]
pub fn BreadcrumbEllipsis(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    // TODO. data_state
    view! {
        <RootEllipsisBtn
            attr:aria-haspopup="menu"
            attr:aria-expanded="false"
            attr:data-state="closed"
        >
            <RootEllipsis attr:role="presentation" attr:aria-hidden="true">
                <Ellipsis class=class />
                <span class="sr-only">More</span>
            </RootEllipsis>
            <span class="sr-only">Toggle menu</span>
        </RootEllipsisBtn>
    }
}
