use leptos::prelude::*;
use leptos_ui::input;

const FLEX_WIDTH_FULL: &str = "flex w-full";
const FILE_STYLES: &str = "file:bg-transparent file:text-sm file:font-medium  file:border-0";
const FOCUS_VISIBLE_RING: &str = "focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring focus-visible:ring-offset-1";
const BORDER_INPUT: &str = "border border-input";
const DISABLED_NOT_ALLOWED: &str = "disabled:cursor-not-allowed disabled:opacity-50";
const RING_OFFSET_BG: &str = "ring-offset-background";
const PLACEHOLDER_MUTED_FOREGROUND: &str = "placeholder:text-muted-foreground";

mod components {
    use super::*;

    input! {Input,
        PLACEHOLDER_MUTED_FOREGROUND,
        FILE_STYLES,
        DISABLED_NOT_ALLOWED,
        FOCUS_VISIBLE_RING,
        RING_OFFSET_BG,
        BORDER_INPUT,
        FLEX_WIDTH_FULL,
        "h-10 rounded-md bg-background px-3 py-2",
        "text-sm text-muted-foreground"
    }
}

pub use components::*;
