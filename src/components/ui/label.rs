use leptos::prelude::*;
use tw_merge::*;

const DISABLED_NOT_ALLOWED_PEER: &str = "peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

#[component]
pub fn Label(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] html_for: String,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        DISABLED_NOT_ALLOWED_PEER,
        "text-sm font-medium leading-none",
        class
    );

    view! {
        <label class=class r#for=html_for>
            {children()}
        </label>
    }
}
