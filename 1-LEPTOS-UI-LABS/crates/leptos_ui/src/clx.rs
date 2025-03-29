use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::atomic::{AtomicUsize, Ordering},
};

pub use leptos::prelude::*;
pub use tw_merge::*;

/// A macro that creates a component with tailwind class merging
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use leptos_ui::clx;
///
/// // Define the component
/// clx! {Card, div, "rounded-lg p-4", "bg-sky-500"} // 🩵
///
/// #[component]
/// pub fn DemoCard() -> impl IntoView {
///     view! {
///         <Card>"Card bg-sky-500 🩵"</Card>
///         <Card class="bg-orange-500">"Card bg-orange-500 🧡"</Card>
///     }
/// }
/// ```
#[macro_export]
macro_rules! clx {
    ($name:ident, $element:ident, $($base_class:expr),+ $(,)?) => {
        #[component]
        pub fn $name(
            #[prop(into, optional)] class: Signal<String>,
            #[prop(into, optional)] style: Option<String>,
            #[prop(optional)] role: Option<&'static str>,
            #[prop(optional)] onclick: Option<&'static str>,
            #[prop(optional)] onclose: Option<&'static str>,
            #[prop(optional)] id: Option<&'static str>,
            #[prop(optional)] tabindex: Option<&'static str>,
            children: Children,
        ) -> impl IntoView {
            let merged_classes = Memo::new(move |_| {
                tw_merge::tw_merge!(tw_merge::tw_join!($($base_class),+), class())
            });

            view! {
                <$element class=merged_classes style=style role=role onclick=onclick onclose=onclose id=id tabindex=tabindex>
                    {children()}
                </$element>
            }
        }
    };
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ NO CHILDREN ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// * Special macro since input doesn't accept children.
#[macro_export]
macro_rules! input {
    ($name:ident, $($base_class:expr),+ $(,)?) => {
        #[component]
        pub fn $name(
            #[prop(into, optional)] class: Signal<String>,
            #[prop(into, optional)] r#type: Option<String>,
            #[prop(optional_no_strip)] value: Option<ReadSignal<String>>,
            #[prop(optional)] placeholder: Option<&'static str>,
            #[prop(optional)] name: Option<&'static str>,
            #[prop(optional)] id: Option<&'static str>,
            #[prop(into, optional)] min: Option<String>,
            #[prop(into, optional)] step: Option<String>,
            #[prop(into, optional)] max: Option<String>,
            #[prop(into, optional)] autofocus: bool,
            #[prop(optional)] autocomplete: Option<&'static str>,
            #[prop(optional)] required: bool,
        ) -> impl IntoView {
            let merged_classes = Memo::new(move |_| {
                tw_merge::tw_merge!(tw_merge::tw_join!($($base_class),+), class())
            });

            view! {
                <input
                    class=merged_classes
                    name=name
                    value=value
                    type=r#type
                    placeholder=placeholder
                    required=required
                    min=min
                    step=step
                    max=max
                    autofocus=autofocus
                    autocomplete=autocomplete
                />
            }
        }
    };
}

// * Special macro for div that don't accept children.
#[macro_export]
macro_rules! div {
    ($name:ident, $($base_class:expr),+ $(,)?) => {
        #[component]
        pub fn $name(
            #[prop(into, optional)] class: Signal<String>,
            #[prop(into, optional)] style: Option<String>,
            #[prop(optional)] onclick: Option<&'static str>,
        ) -> impl IntoView {
            let merged_classes = Memo::new(move |_| {
                tw_merge::tw_merge!(tw_merge::tw_join!($($base_class),+), class())
            });

            view! {
                <div
                    class=merged_classes
                    style=style
                    onclick=onclick
                />
            }
        }
    };
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub struct Utils;

impl Utils {
    pub fn use_random_id() -> String {
        let mut hasher = DefaultHasher::new();

        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        counter.hash(&mut hasher);

        format!("_gen_id_{}", hasher.finish())
    }

    pub fn use_random_transition_name() -> String {
        let random_id = Utils::use_random_id();
        format!("view-transition-name: {}", random_id)
    }
}

#[macro_export]
macro_rules! transition {
    ($name:ident, $element:ident, $($base_class:expr),+ $(,)?) => {
        #[component]
        pub fn $name(
            #[prop(into, optional)] class: Signal<String>,
            #[prop(optional)] role: Option<&'static str>,
            #[prop(optional)] onclick: Option<&'static str>,
            #[prop(optional)] onclose: Option<&'static str>,
            #[prop(optional)] id: Option<&'static str>,
            #[prop(optional)] tabindex: Option<&'static str>,
            children: Children,
        ) -> impl IntoView {
            let merged_classes = Memo::new(move |_| {
                tw_merge::tw_merge!(tw_merge::tw_join!($($base_class),+), class())
            });

            let random_name = Utils::use_random_transition_name();

            view! {
                <$element class=merged_classes style=random_name role=role onclick=onclick onclose=onclose id=id tabindex=tabindex>
                    {children()}
                </$element>
            }
        }
    };
}
