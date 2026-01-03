use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {BottomNav, nav, "z-50 mx-auto w-full max-w-lg border-t border-border bg-background"}
    clx! {BottomNavGrid, div,
        "grid grid-flow-col auto-cols-fr h-16 font-medium",
        // * SHORTFIX 🚑 iOS Safari: reduce height for tighter nav
        "supports-[-webkit-touch-callout:none]:h-14"
    }
    clx! {BottomNavButton, button,
        "inline-flex flex-col justify-center items-center px-5 group [&_svg]:mb-2 [&_svg]:text-muted-foreground hover:[&_svg]:text-primary aria-[current=page]:[&_svg]:text-primary",
        // * SHORTFIX 🚑 iOS Safari: push icons closer to home indicator
        "supports-[-webkit-touch-callout:none]:justify-end supports-[-webkit-touch-callout:none]:pb-0 supports-[-webkit-touch-callout:none]:translate-y-1"
    }
    clx! {BottomNavLabel, span, "text-sm text-muted-foreground group-hover:text-primary group-aria-[current=page]:text-primary"}
}

pub use components::*;
