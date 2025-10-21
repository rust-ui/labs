use icons::{ChevronLeft, ChevronRight, Ellipsis};
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::ui::button::{ButtonClass, ButtonSize, ButtonVariant};
use crate::utils::hooks::use_pagination::{PaginationContext, use_pagination};
use crate::utils::query::QUERY;

mod components {
    use super::*;
    clx! {PaginationNav, nav, "flex justify-center mx-auto w-full"}
    clx! {PaginationList, ul, "flex flex-row gap-1 items-center [&_li:nth-last-child(2):has(a[aria-current=page])~li:last-child]:opacity-0 [&_li:nth-last-child(2):has(a[aria-current=page])~li:last-child]:pointer-events-none"}
    clx! {PaginationItem, li, ""}
    clx! {EllipsisRoot, span, "flex justify-center items-center size-9"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
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

pub enum PageDirection {
    Previous,
    Next,
}

#[component]
pub fn PaginationNavButton(direction: PageDirection) -> impl IntoView {
    let ctx = use_context::<PaginationContext>();

    let is_previous = matches!(direction, PageDirection::Previous);

    let (href, is_disabled) = if let Some(ctx) = ctx {
        let href = Signal::derive(move || {
            let current = ctx.current_page.get();
            if is_previous {
                if current > 1 {
                    ctx.page_href.run(current - 1)
                } else {
                    "#".to_string()
                }
            } else {
                ctx.page_href.run(current + 1)
            }
        });
        let is_disabled = Signal::derive(move || {
            if is_previous {
                ctx.current_page.get() <= 1
            } else {
                false
            }
        });
        (href, is_disabled)
    } else {
        (Signal::derive(|| "#".to_string()), Signal::derive(|| true))
    };

    let button_class = ButtonClass {
        variant: ButtonVariant::Ghost,
        size: ButtonSize::Default,
    };

    let (aria_label, extra_class) = if is_previous {
        ("Go to previous page", "sm:pl-2.5")
    } else {
        ("Go to next page", "sm:pr-2.5")
    };

    view! {
        <a
            href=href
            class=button_class.with_class(extra_class)
            class:opacity-0=is_disabled
            class:pointer-events-none=is_disabled
            aria-label=aria_label
        >
            {move || {
                if is_previous {
                    view! { <ChevronLeft /> }.into_any()
                } else {
                    view! { <ChevronRight /> }.into_any()
                }
            }}

        </a>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn PaginationLink(
    page: u32,
    children: Children,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let ctx = use_context::<PaginationContext>();

    let href = if let Some(ctx) = ctx.clone() {
        Signal::derive(move || ctx.page_href.run(page))
    } else {
        Signal::derive(|| "#".to_string())
    };

    let aria_current = if let Some(ctx) = ctx {
        Signal::derive(move || {
            if ctx.current_page.get() == page {
                QUERY::PAGE
            } else {
                ""
            }
        })
    } else {
        Signal::derive(|| "")
    };

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
            {children()}
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
