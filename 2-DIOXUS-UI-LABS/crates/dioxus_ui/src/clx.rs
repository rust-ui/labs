use dioxus::prelude::*;
pub use tw_merge::*;

/// A macro for creating UI components with tailwind classes in Dioxus.
///
/// This macro simplifies the creation of components that use tailwind classes
/// by handling the merging of base classes with user-provided classes.
///
/// # Example
///
/// ```rust
/// use dioxus_ui::clx;
///
/// mod components {
///     use super::*;
///     clx! {Card, div, "rounded-lg border bg-card shadow p-4 w-full"}
///     clx! {CardHeader, div, "flex flex-col space-y-1.5"}
///     clx! {CardTitle, h3, "text-2xl font-semibold leading-none tracking-tight"}
/// }
///
/// pub use components::*;
/// ```
#[macro_export]
macro_rules! clx {
    ($name:ident, $element:ident, $($base_class:expr),+ $(,)?) => {
        #[component]
        pub fn $name(
            #[props(into, optional)] class: Option<String>,
            #[props(optional)] role: Option<&'static str>,
            #[props(optional)] style: Option<&'static str>,
            children: Element,
        ) -> Element {
            let merged_class = use_memo(move || {
                tw_merge::tw_merge!(
                    tw_merge::tw_join!($($base_class),+),
                    class.as_deref().unwrap_or("")
                )
            });

            rsx! {
                $element {
                    class: "{merged_class}",
                    role: role,
                    style: style,
                    {children}
                }
            }
        }
    };
}

/// Helper macro to join multiple tailwind classes into a single string.
///
/// This is used internally by the `clx!` macro but can also be used separately.
#[macro_export]
macro_rules! tw_join {
    ($($class:expr),+) => {
        [$($class),+].join(" ")
    };
}
