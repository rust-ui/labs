use dioxus::prelude::*;
use tw_merge::*;

// TODO 💪 Loading state (demo_use_timeout_fn.rs and demo_button.rs)

#[component]
pub fn Button(
    #[props(into, optional)] variant: Option<ButtonVariant>,
    #[props(into, optional)] size: Option<ButtonSize>,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] id: Option<String>,
    #[props(into, optional)] formmethod: Option<String>,
    #[props(into, optional)] value: Option<String>,
    #[props(into, optional)] role: Option<String>,
    #[props(into, optional)] disabled: Option<bool>,
    #[props(into, optional)] r#type: Option<String>,
    children: Element,
) -> Element {
    let class = use_memo(move || {
        let variant = variant.unwrap_or(ButtonVariant::Default);
        let size = size.unwrap_or(ButtonSize::Default);
        let button = ButtonClass { variant, size };
        button.with_class(class.as_deref().unwrap_or(""))
    });

    rsx! {
        button {
            class: "{class}",
            disabled,
            id,
            role,
            r#type,
            formmethod,
            value,
            {children}
        }
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🧬 STRUCT 🧬                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[derive(TwClass, Default)]
#[tw(
    class = "inline-flex items-center justify-center text-sm font-medium transition-colors rounded-md w-fit whitespace-nowrap focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"
)]
pub struct ButtonClass {
    variant: ButtonVariant,
    size: ButtonSize,
}

#[allow(dead_code)]
#[derive(TwVariant, PartialEq)]
pub enum ButtonVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "bg-destructive text-destructive-foreground hover:bg-destructive/90")]
    Destructive,
    #[tw(class = "bg-warning text-warning-foreground hover:bg-warning/90")]
    Warning,
    #[tw(class = "bg-success text-success-foreground hover:bg-success/90")]
    Success,
    #[tw(class = "border border-input bg-background hover:bg-accent hover:text-accent-foreground")]
    Outline,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "underline-offset-4 hover:underline")]
    Link,
}

#[derive(TwVariant, PartialEq)]
pub enum ButtonSize {
    #[tw(default, class = "px-4 py-2 h-9")]
    Default,
    #[tw(class = "h-8 px-3")]
    Sm,
    #[tw(class = "h-10 px-8")]
    Lg,
    #[tw(class = "size-8")]
    Icon,
}
