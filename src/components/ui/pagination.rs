use icons::{ChevronLeft, ChevronRight, Ellipsis};
use leptos::prelude::*;
use leptos_ui::clx;
use strum::Display;
use tw_merge::*;

use crate::components::hooks::use_pagination::{PaginationContext, use_pagination};
use crate::components::ui::button::{ButtonClass, ButtonSize, ButtonVariant};

mod components {
    use super::*;
    clx! {PaginationNav, nav, "flex justify-center mx-auto w-full"}
    clx! {PaginationList, ul, "flex flex-row gap-1 items-center [&_li:nth-last-child(2):has(a[aria-current=page])~li:last-child]:opacity-0 [&_li:nth-last-child(2):has(a[aria-current=page])~li:last-child]:pointer-events-none"}
    clx! {PaginationItem, li, ""}
    clx! {EllipsisRoot, span, "flex justify-center items-center size-9"}
}

pub use components::*;

/* ========================================================== */
/*                    ✨ COMPONENTS ✨                        */
/* ========================================================== */

#[component]
pub fn Pagination(children: Children) -> impl IntoView {
    let ctx = use_pagination();
    provide_context(ctx);

    view! { <PaginationNav>{children()}</PaginationNav> }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum PageDirection {
    Previous,
    Next,
}

#[component]
pub fn PaginationNavButton(direction: PageDirection) -> impl IntoView {
    let ctx = use_context::<PaginationContext>().expect("PaginationContext not found");

    let (href, is_disabled, icon) = match direction {
        PageDirection::Previous => (
            ctx.prev_href,
            ctx.is_first_page,
            view! { <ChevronLeft /> }.into_any(),
        ),
        PageDirection::Next => (
            ctx.next_href,
            Signal::derive(|| false),
            view! { <ChevronRight /> }.into_any(),
        ),
    };

    let button_class = ButtonClass {
        variant: ButtonVariant::Ghost,
        size: ButtonSize::Default,
    };

    view! {
        <a
            href=href
            class=button_class.with_class("")
            class:opacity-0=is_disabled
            class:pointer-events-none=is_disabled
            aria-label=format!("Go to {} page", direction)
        >
            {icon}
        </a>
    }
}

#[component]
pub fn PaginationLink(page: u32, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = use_context::<PaginationContext>().expect("PaginationContext not found");

    let href = Signal::derive(move || ctx.page_href.run(page));
    let aria_current = move || ctx.aria_current.run(page);

    let button_class = ButtonClass {
        variant: ButtonVariant::Ghost,
        size: ButtonSize::Icon,
    };

    let merged_class = button_class.with_class(tw_merge!(
        "aria-[current=page]:bg-primary aria-[current=page]:text-primary-foreground aria-[current=page]:hover:bg-primary/90",
        class
    ));

    view! {
        <a href=href aria-current=aria_current class=merged_class>
            {page}
        </a>
    }
}

#[component]
pub fn PaginationEllipsis() -> impl IntoView {
    view! {
        <EllipsisRoot attr:aria-hidden="true">
            <Ellipsis class="size-4" />
            <span class="sr-only">More pages</span>
        </EllipsisRoot>
    }
}
