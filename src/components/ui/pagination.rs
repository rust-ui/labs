use icons::{ChevronLeft, ChevronRight, Ellipsis};
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::ui::button::{ButtonClass, ButtonSize, ButtonVariant};
use crate::utils::hooks::use_pagination::{PaginationContext, use_pagination};

mod components {
    use super::*;
    clx! {PaginationNav, nav, "flex justify-center mx-auto w-full"}
    clx! {PaginationList, ul, "flex flex-row gap-1 items-center"}
    clx! {PaginationItem, li, ""}
    clx! {EllipsisRoot, span, "flex justify-center items-center size-9"}
}

pub use components::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                    ✨ COMPONENTS ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn Pagination(
    #[prop(into, optional)] query_key: String,
    #[prop(into, optional)] max_pages: u32,
    children: Children,
) -> impl IntoView {
    if !query_key.is_empty() {
        let ctx = use_pagination(query_key, max_pages);
        provide_context(ctx);
    }

    view! { <PaginationNav>{children()}</PaginationNav> }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

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
                "page"
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
pub fn PaginationNext() -> impl IntoView {
    let ctx = use_context::<PaginationContext>();

    let (href, is_disabled) = if let Some(ctx) = ctx {
        let href = Signal::derive(move || {
            let current = ctx.current_page.get();
            if current < ctx.max_pages {
                ctx.page_href.run(current + 1)
            } else {
                "#".to_string()
            }
        });
        let is_disabled = Signal::derive(move || ctx.current_page.get() >= ctx.max_pages);
        (href, is_disabled)
    } else {
        (Signal::derive(|| "#".to_string()), Signal::derive(|| true))
    };

    let button_class = ButtonClass {
        variant: ButtonVariant::Ghost,
        size: ButtonSize::Default,
    };

    view! {
        <a
            href=href
            class=button_class.with_class("sm:pr-2.5")
            class:opacity-0=is_disabled
            class:pointer-events-none=is_disabled
            aria-label="Go to next page"
        >
            <ChevronRight />
        </a>
    }
}

#[component]
pub fn PaginationPrevious() -> impl IntoView {
    let ctx = use_context::<PaginationContext>();

    let (href, is_disabled) = if let Some(ctx) = ctx {
        let href = Signal::derive(move || {
            let current = ctx.current_page.get();
            if current > 1 {
                ctx.page_href.run(current - 1)
            } else {
                "#".to_string()
            }
        });
        let is_disabled = Signal::derive(move || ctx.current_page.get() <= 1);
        (href, is_disabled)
    } else {
        (Signal::derive(|| "#".to_string()), Signal::derive(|| true))
    };

    let button_class = ButtonClass {
        variant: ButtonVariant::Ghost,
        size: ButtonSize::Default,
    };

    view! {
        <a
            href=href
            class=button_class.with_class("sm:pl-2.5")
            class:opacity-0=is_disabled
            class:pointer-events-none=is_disabled
            aria-label="Go to previous page"
        >
            <ChevronLeft />
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
