use leptos::prelude::*;
use tw_merge::*;

#[component]
pub fn Label(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] html_for: String,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        "flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50",
        class
    );

    view! {
        <label class=class r#for=html_for>
            {children()}
        </label>
    }
}
