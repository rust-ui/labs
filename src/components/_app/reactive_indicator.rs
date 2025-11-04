use leptos::prelude::*;

#[cfg(not(feature = "ssr"))]
const TODO_TIMEOUT_MS: u64 = 100;

#[component]
pub fn ReactiveIndicator() -> impl IntoView {
    let show = RwSignal::new(false);
    let is_ready = RwSignal::new(false);

    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        // Check if we're running on localhost
        let window = web_sys::window().expect("no global window");
        let location = window.location();
        let hostname = location.hostname().unwrap_or_default();

        if hostname == "localhost" || hostname == "127.0.0.1" {
            show.set(true);

            set_timeout(
                move || {
                    is_ready.set(true);
                },
                std::time::Duration::from_millis(TODO_TIMEOUT_MS),
            );
        }
    });

    view! {
        <Show when=move || show.get() fallback=|| ()>
            <div
                data-name="ReactiveIndicator"
                class=move || {
                    let base_class = "size-3 fixed bottom-3 right-5 z-999 rounded-full transition-colors duration-300 ease-in-out";
                    let color_class = if is_ready.get() { "bg-green-500" } else { "bg-orange-500" };
                    format!("{base_class} {color_class}")
                }
            />
        </Show>
    }
}
