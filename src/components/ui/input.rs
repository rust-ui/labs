use leptos::prelude::*;
use leptos_ui::void;

mod components {
    use super::*;

    void! {Input, input,
        "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input flex h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        "focus-visible:border-ring focus-visible:ring-ring/50",
        "focus-visible:ring-2", // TODO. Port tw_merge to Tailwind V4.
     // "focus-visible:ring-[3px]", // TODO. Port tw_merge to Tailwind V4.
        "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
        //
        // Added (TW V4)
        "read-only:bg-muted",
    }
}

pub use components::*;
