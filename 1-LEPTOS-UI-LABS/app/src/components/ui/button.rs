use leptos::prelude::*;
use tw_merge::*;

// TODO 💪 Loading state (demo_use_timeout_fn.rs and demo_button.rs)

#[component]
pub fn Button(
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] size: Signal<ButtonSize>,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] id: Signal<String>,
    #[prop(into, optional)] formmethod: Signal<String>,
    #[prop(into, optional)] value: Signal<String>,
    #[prop(into, optional)] role: Signal<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] r#type: Signal<String>,
    #[prop(into, optional)] onclick_string: Option<String>,
    // #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        let variant = variant.get();
        let size = size.get();
        let button = ButtonClass { variant, size };
        button.with_class(class.get())
    });

    view! {
        <button
            // {..attributes}
            class=class
            disabled=disabled
            id=id
            role=role
            type=r#type
            formmethod=formmethod
            value=value
            onclick=onclick_string
        >
            {children()}
        </button>
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

#[derive(TwVariant)]
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

#[derive(TwVariant)]
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
