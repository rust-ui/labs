use leptos::prelude::*;
use leptos_ui::div;

const TIMEOUT_MS: u64 = 100;

#[component]
pub fn ReactiveIndicator() -> impl IntoView {
    let is_reactive = RwSignal::new(false);

    div! {Indicator, "size-3 rounded-full transition-colors duration-300 ease-in-out"}

    let class = Signal::derive(move || {
        if is_reactive.get() {
            "bg-green-500".to_string()
        } else {
            "bg-orange-500".to_string()
        }
    });

    Effect::new(move |_| {
        set_timeout(
            move || {
                is_reactive.set(true);
            },
            std::time::Duration::from_millis(TIMEOUT_MS),
        );
    });

    view! { <Indicator class=class /> }
}
