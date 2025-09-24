use icons::{ChevronLeft, ChevronRight, Ellipsis};
use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Pagination, nav, "flex justify-center mx-auto w-full"}
    clx! {PaginationContent, ul, "flex flex-row gap-1 items-center"}
    clx! {PaginationItem, li, ""}
    clx! {EllipsisRoot, span, "flex justify-center items-center size-9"}

    const COMMON_CLASSES: &str = "inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md outline-none  hover:bg-accent hover:text-accent-foreground transition-all  disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none  [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0  aria-invalid:ring-destructive/20 aria-invalid:border-destructive dark:aria-invalid:ring-destructive/40  focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]";

    // TODO. Merge common classes first.

    clx! {PaginationLink, a, COMMON_CLASSES, "gap-2  size-9 dark:hover:bg-accent/50"}

    clx! {PaginationActive, a, COMMON_CLASSES, "gap-2 border  bg-background shadow-xs size-9 dark:bg-input/30 dark:border-input dark:hover:bg-input/50"}

    clx! {RootPrevious, a, COMMON_CLASSES,  "py-2 has-[>svg]:px-3 dark:hover:bg-accent/50"}

    clx! {RootNext, a, COMMON_CLASSES, "h-9  has-[>svg]:px-3 dark:hover:bg-accent/50"}
}

pub use components::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn PaginationNext(href: &'static str) -> impl IntoView {
    view! {
        <RootNext
            class="gap-1 py-2 px-2.5 sm:pr-2.5"
            attr:aria-label="Go to next page"
            attr:href=href
        >
            <span class="hidden sm:block">Next</span>
            <ChevronRight />
        </RootNext>
    }
}

#[component]
pub fn PaginationPrevious(href: &'static str) -> impl IntoView {
    view! {
        <RootPrevious
            class="gap-1 px-2.5 h-9 sm:pl-2.5"
            attr:aria-label="Go to previous page"
            attr:href=href
        >
            <ChevronLeft />
            <span class="hidden sm:block">Previous</span>
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
