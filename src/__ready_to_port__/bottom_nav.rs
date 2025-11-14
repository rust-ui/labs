use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {BottomNav, nav, "mx-auto w-full max-w-lg h-16 bg-card border-t border-border"}
    clx! {BottomNavGrid, div, "grid grid-cols-4 h-full font-medium"}
    clx! {BottomNavButton, button, "inline-flex flex-col justify-center items-center px-5 group [&_svg]:mb-2 [&_svg]:text-muted-foreground hover:[&_svg]:text-primary aria-[current=page]:[&_svg]:text-primary"}
    clx! {BottomNavLabel, span, "text-sm text-muted-foreground group-hover:text-primary group-aria-[current=page]:text-primary"}
}

pub use components::*;
