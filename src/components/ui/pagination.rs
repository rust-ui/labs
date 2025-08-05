use icons::{ChevronLeft, ChevronRight, Ellipsis};
use leptos::prelude::*;
use leptos_ui::{a, clx};

mod components {
    use super::*;
    clx! {Pagination, nav, "flex justify-center mx-auto w-full"}
    clx! {PaginationContent, ul, "flex flex-row gap-1 items-center"}
    clx! {PaginationItem, li, ""}
    clx! {EllipsisRoot, span, "flex justify-center items-center size-9"}

    a! {PaginationLink, "inline-flex gap-2 justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-all outline-none disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive size-9 dark:aria-invalid:ring-destructive/40 dark:hover:bg-accent/50 hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"}

    // TODO. Merge with the one above

    a! {PaginationActive, "inline-flex gap-2 justify-center items-center text-sm font-medium whitespace-nowrap rounded-md border transition-all outline-none disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive bg-background shadow-xs size-9 dark:aria-invalid:ring-destructive/40 dark:bg-input/30 dark:border-input dark:hover:bg-input/50 hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"}

    a! {RootPrevious, "inline-flex gap-1 justify-center items-center py-2 px-2.5 h-9 text-sm font-medium whitespace-nowrap rounded-md transition-all outline-none sm:pl-2.5 disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive has-[>svg]:px-3 dark:aria-invalid:ring-destructive/40 dark:hover:bg-accent/50 hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"}

    a! {RootNext, "inline-flex gap-1 justify-center items-center py-2 px-2.5 h-9 text-sm font-medium whitespace-nowrap rounded-md transition-all outline-none sm:pr-2.5 disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive has-[>svg]:px-3 dark:aria-invalid:ring-destructive/40 dark:hover:bg-accent/50 hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"}
}

pub use components::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn PaginationNext(href: &'static str) -> impl IntoView {
    view! {
        <RootNext aria_label="Go to next page" href=href>
            <span class="hidden sm:block">Next</span>
            <ChevronRight />
        </RootNext>
    }
}

#[component]
pub fn PaginationPrevious(href: &'static str) -> impl IntoView {
    view! {
        <RootPrevious aria_label="Go to previous page" href=href>
            <ChevronLeft />
            <span class="hidden sm:block">Previous</span>
        </RootPrevious>
    }
}

#[component]
pub fn PaginationEllipsis() -> impl IntoView {
    view! {
        <EllipsisRoot aria_hidden="true">
            <Ellipsis class="size-4" />
            <span class="sr-only">More pages</span>
        </EllipsisRoot>
    }
}
