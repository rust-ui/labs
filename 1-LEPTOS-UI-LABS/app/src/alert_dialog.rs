use leptos::prelude::*;
use leptos_meta::StyleSheet;
#[component]
pub fn JsAlertIntro() -> impl IntoView {
    view! {
        <div>
            <h1>JS Alert Intro</h1>
            <p>This is a simple example of how to use JS alerts in Leptos UI.</p>
            <button onclick=move |_| alert("Hello, world!").into_view()>
                Click me!
            </button>
        </div>
    }
}
