use leptos::prelude::*;
use leptos_ui::input;

use crate::components::ui::_styles::STYLES;

mod components {
    use super::*;
    input! {Input,
        STYLES.PLACEHOLDER_MUTED_FOREGROUND,
        STYLES.FILE_STYLES,
        STYLES.DISABLED_NOT_ALLOWED,
        STYLES.FOCUS_VISIBLE_RING,
        STYLES.RING_OFFSET_BG,
        STYLES.BORDER_INPUT,
        STYLES.FLEX_WIDTH_FULL,
        "h-10 rounded-md bg-background px-3 py-2 text-sm",
    }
}

pub use components::*;
