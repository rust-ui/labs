use icons::{ChevronLeft, ChevronRight, Ellipsis};
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::ui::button::{ButtonClass, ButtonSize, ButtonVariant};
use crate::utils::query::QueryUtils;

const COMMON_CLASSES: &str = "inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md outline-none  hover:bg-accent hover:text-accent-foreground transition-all  disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none  [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0  aria-invalid:ring-destructive/20 aria-invalid:border-destructive dark:aria-invalid:ring-destructive/40  focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]";

mod components {
    use super::*;
    clx! {PaginationNav, nav, "flex justify-center mx-auto w-full"}
    clx! {PaginationList, ul, "flex flex-row gap-1 items-center"}
    clx! {PaginationItem, li, ""}
    clx! {EllipsisRoot, span, "flex justify-center items-center size-9"}

    // TODO. Merge common classes first.

    clx! {RootPrevious, a, COMMON_CLASSES,  "gap-1 px-2.5  h-9 py-2 has-[>svg]:px-3 dark:hover:bg-accent/50"}
    clx! {RootNext, a, COMMON_CLASSES, "gap-1 px-2.5  h-9 py-2 has-[>svg]:px-3 dark:hover:bg-accent/50"}
}

pub use components::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                       ✨ CONTEXT ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(Clone)]
struct PaginationContext {
    current_page: Memo<u32>,
    page_href: Callback<u32, String>,
    max_pages: u32,
}

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
        let location = use_location();
        let current_page_str = QueryUtils::extract(query_key.clone());

        let current_page = Memo::new(move |_| current_page_str().parse::<u32>().unwrap_or(1));

        let page_href = Callback::new(move |page: u32| {
            location.query.with(|q| {
                let demo_param = q
                    .get("demo")
                    .map(|d| format!("demo={}&", d))
                    .unwrap_or_default();
                format!("?{}{}={}", demo_param, query_key, page)
            })
        });

        let ctx = PaginationContext {
            current_page,
            page_href,
            max_pages,
        };

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
    let merged_class = tw_merge!(
        COMMON_CLASSES,
        "gap-2  size-9 dark:hover:bg-accent/50 aria-[current=page]:bg-primary aria-[current=page]:text-primary-foreground aria-[current=page]:shadow-xs aria-[current=page]:hover:bg-primary/90",
        class
    );

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

    view! {
        <a data-name="PaginationLink" href=href aria-current=aria_current class=merged_class>
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

    view! {
        <RootNext
            class="sm:pr-2.5"
            class:opacity-0=is_disabled
            class:pointer-events-none=is_disabled
            attr:aria-label="Go to next page"
            attr:href=href
        >
            <ChevronRight />
        </RootNext>
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

    view! {
        <RootPrevious
            class="sm:pl-2.5"
            class:opacity-0=is_disabled
            class:pointer-events-none=is_disabled
            attr:aria-label="Go to previous page"
            attr:href=href
        >
            <ChevronLeft />
        </RootPrevious>
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

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// TODO. Merge properly with Button
#[component]
pub fn PaginationLinkWithButton(
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] size: Signal<ButtonSize>,
    #[prop(into, optional)] class: Signal<String>,
    // TODO _is_active.
    #[prop(into, optional)] _is_active: Option<bool>,
    #[prop(into)] href: &'static str,
    children: Children,
) -> impl IntoView {
    // TODO. should be ButtonVariant::Ghost OR ButtonVariant::Outline if is_active
    let class = Memo::new(move |_| {
        let variant = variant.get();
        let size = size.get();
        let button = ButtonClass { variant, size };
        button.with_class(class.get())
    });

    view! {
        <a class=class href=href>
            {children()}
        </a>
    }
}
