use leptos::prelude::*;
use leptos_ui::variants;

// TODO 💪 Loading state (demo_use_timeout_fn.rs and demo_button.rs)

variants! {
    Button {
        base: "inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-colors w-fit focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 [&_svg:not(:last-child)]:mr-2 [&_svg:not(:first-child)]:ml-2 [&_svg:not([class*='size-'])]:size-4 hover:cursor-pointer",
        variants: {
            variant: {
                Default: "bg-primary text-primary-foreground hover:bg-primary/90",
                Secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
                Accent: "bg-accent text-accent-foreground hover:bg-accent/80",
                Destructive: "bg-destructive text-destructive-foreground hover:bg-destructive/90",
                Warning: "bg-warning text-warning-foreground hover:bg-warning/90",
                Success: "bg-success text-success-foreground hover:bg-success/90",
                Outline: "border bg-background border-input hover:bg-accent hover:text-accent-foreground",
                Bordered: "bg-transparent border border-zinc-200 text-muted-foreground",
                Ghost: "hover:bg-accent hover:text-accent-foreground",
                Link: "underline-offset-4 hover:underline",
            },
            size: {
                Default: "h-9 px-4 py-2 has-[>svg]:px-3",
                Sm: "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5",
                Lg: "h-10 rounded-md px-6 has-[>svg]:px-4",
                Icon: "size-8",
                Mobile: "px-6 py-3 rounded-[24px]",
                Badge: "px-2.5 py-0.5 text-xs"
            }
        },
        component: {
            element: button,
            href_support: true
        }
    }
}
