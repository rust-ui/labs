use leptos::prelude::*;
use tw_merge::*;

#[component]
pub fn SvgIcon(#[prop(into, optional)] class: Signal<String>, children: Children) -> impl IntoView {
    let class = Memo::new(move |_| tw_merge!("", class()));

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"

            class=class
        >
            {children()}
        </svg>
    }
}
