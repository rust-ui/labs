use icons::{ChevronLeft, ChevronRight, Ellipsis};
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::ui::button::{ButtonClass, ButtonSize, ButtonVariant};

mod components {
    use super::*;
    clx! {Pagination, nav, "flex justify-center mx-auto w-full"}
    clx! {PaginationList, ul, "flex flex-row gap-1 items-center"}
    clx! {PaginationItem, li, ""}
    clx! {EllipsisRoot, span, "flex justify-center items-center size-9"}

    const COMMON_CLASSES: &str = "inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md outline-none  hover:bg-accent hover:text-accent-foreground transition-all  disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none  [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0  aria-invalid:ring-destructive/20 aria-invalid:border-destructive dark:aria-invalid:ring-destructive/40  focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]";

    // TODO. Merge common classes first.

    clx! {PaginationLink, a, COMMON_CLASSES, "gap-2  size-9 dark:hover:bg-accent/50 aria-[current=page]:bg-primary aria-[current=page]:text-primary-foreground aria-[current=page]:shadow-xs aria-[current=page]:hover:bg-primary/90"}

    clx! {RootPrevious, a, COMMON_CLASSES,  "gap-1 px-2.5  h-9 py-2 has-[>svg]:px-3 dark:hover:bg-accent/50"}
    clx! {RootNext, a, COMMON_CLASSES, "gap-1 px-2.5  h-9 py-2 has-[>svg]:px-3 dark:hover:bg-accent/50"}
}

pub use components::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn PaginationNext(href: &'static str) -> impl IntoView {
    view! {
        <RootNext class="sm:pr-2.5" attr:aria-label="Go to next page" attr:href=href>
            <ChevronRight />
        </RootNext>
    }
}

#[component]
pub fn PaginationPrevious(href: &'static str) -> impl IntoView {
    view! {
        <RootPrevious class="sm:pl-2.5" attr:aria-label="Go to previous page" attr:href=href>
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
