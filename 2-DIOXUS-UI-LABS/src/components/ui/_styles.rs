#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Styles {
    pub RELATIVE: &'static str,
    pub ABSOLUTE: &'static str,
    //
    pub BG_MUTED: &'static str,
    pub BG_TRANSPARENT: &'static str,
    pub BLOCK_WIDTH_FULL: &'static str,
    pub BLOCK_INSET_ZERO: &'static str,
    pub BORDER_INPUT: &'static str,
    pub BORDER_PRIMARY: &'static str,
    pub CHECKBOX_DATA_STATE_PRIMARY: &'static str,
    pub DISABLED_NOT_ALLOWED: &'static str,
    pub DISABLED_EVENTS_NONE: &'static str,
    pub DISABLED_NOT_ALLOWED_PEER: &'static str,
    pub FLEX_WIDTH_FULL: &'static str,
    pub FLEX_ITEMS_CENTER: &'static str,
    pub FLEX_ITEMS_JUSTIFY_CENTER: &'static str,
    pub FULL_CENTER_INLINE: &'static str,
    pub FIELD_SIZING_CONTENT: &'static str,
    pub FOCUS_VISIBLE_RING: &'static str,
    pub FOCUS_VISIBLE_BG_ACCCENT_70: &'static str,
    pub FILE_STYLES: &'static str,
    pub GRID_START: &'static str,
    pub HOVER_BG_ACCENT: &'static str,
    pub RING_OFFSET_BG: &'static str,
    pub PLACEHOLDER_MUTED_FOREGROUND: &'static str,
    pub TEXT_MUTED_FOREGROUND: &'static str,
    pub TRANSITION_COLORS: &'static str,
    pub SIZE_FULL: &'static str,
    pub WIDTH_FIT: &'static str,
    pub WIDTH_FULL: &'static str,
    // MODALS
    pub DIALOG_BACKDROP: &'static str,
    pub DIALOG_OPEN_STATE: &'static str,
    pub DIALOG_OPACITY_TRANSITION: &'static str,
    // DATA STATES
    pub DATA_STATE_ON_TOGGLE: &'static str,
    // OTHERS
    pub TOAST: &'static str,
    pub TOAST_GROUP: &'static str, // TODO.
}

pub const STYLES: Styles = Styles {
    RELATIVE: "relative",
    ABSOLUTE: "absolute",
    //
    BG_MUTED: "bg-muted",
    BG_TRANSPARENT: "bg-transparent",
    BLOCK_WIDTH_FULL: "block w-full",
    BLOCK_INSET_ZERO: "block inset-0",
    BORDER_INPUT: "border border-input",
    BORDER_PRIMARY: "border border-primary",
    CHECKBOX_DATA_STATE_PRIMARY:
        "data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground",
    DISABLED_NOT_ALLOWED: "disabled:cursor-not-allowed disabled:opacity-50",
    DISABLED_EVENTS_NONE: "disabled:pointer-events-none disabled:opacity-50",
    DISABLED_NOT_ALLOWED_PEER: "peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
    FLEX_WIDTH_FULL: "flex w-full",
    FLEX_ITEMS_CENTER: "flex items-center",
    FLEX_ITEMS_JUSTIFY_CENTER: "flex items-center justify-center",
    FULL_CENTER_INLINE: "inline-flex items-center justify-center",
    FIELD_SIZING_CONTENT: "field-sizing-content",
    FILE_STYLES: "file:bg-transparent file:text-sm file:font-medium  file:border-0",
    FOCUS_VISIBLE_RING:
        "focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring focus-visible:ring-offset-1",
    FOCUS_VISIBLE_BG_ACCCENT_70: "focus-visible:outline-none focus-visible:bg-accent/70",
    GRID_START: "grid w-full grid-cols-1 items-start",
    HOVER_BG_ACCENT: "hover:bg-accent",
    RING_OFFSET_BG: "ring-offset-background",
    PLACEHOLDER_MUTED_FOREGROUND: "placeholder:text-muted-foreground",
    TEXT_MUTED_FOREGROUND: "text-muted-foreground",
    TRANSITION_COLORS: "transition-colors",
    SIZE_FULL: "h-full w-full",
    WIDTH_FIT: "w-fit",
    WIDTH_FULL: "w-full",
    // MODALS
    DIALOG_BACKDROP: "backdrop:backdrop-blur-sm",
    DIALOG_OPEN_STATE: "[&:not([open])]:pointer-events-none [&[open]]:translate-y-0 [&[open]]:opacity-100",
    DIALOG_OPACITY_TRANSITION: "opacity-0 transition-[opacity,transform",
    // DATA STATES
    DATA_STATE_ON_TOGGLE: "data-[state=on]:bg-accent data-[state=on]:text-accent-foreground",
    // OTHERS
    TOAST: "absolute bottom-0 left-0 w-full h-1 bg-current origin-left !ml-0",
    TOAST_GROUP:
        "group-hover/toast:[animation-play-state:paused] group-focus/toast:[animation-play-state:paused]",
};
