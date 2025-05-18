use leptos::prelude::*;

#[component]
pub fn ReactiveIndicator() -> impl IntoView {
    let (is_ready, set_is_ready) = signal(false);

    const TODO_TIMEOUT_MS: u64 = 500; // TODO. Maybe reduce this

    Effect::new(move |_| {
        // This effect will run once when the component mounts
        set_timeout(
            move || {
                set_is_ready.set(true);
            },
            std::time::Duration::from_millis(TODO_TIMEOUT_MS),
        );
    });

    let class = move || {
        let base_class = "size-3 rounded-full transition-colors duration-300 ease-in-out";
        let color_class = if is_ready.get() {
            "bg-green-500"
        } else {
            "bg-orange-500"
        };
        format!("{base_class} {color_class}")
    };

    view! { <div class=class></div> }
}
