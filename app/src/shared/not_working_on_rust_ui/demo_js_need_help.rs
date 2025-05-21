use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoNeedHelp() -> impl IntoView {
    view! {
        <Stylesheet id="need-help" href="/components/need_help.css" />

        <div class="container">
            <button class="help-btn">
                <svg width="32" height="32" viewBox="0 0 32 32" fill="none">
                    <path
                        d="M3 17c0-5.6 4.4-10 10-10s10 4.4 10 10m-18 0v6c0 1.2.8 2 2 2h4v-8h-4m14 0v6c0 1.2-.8 2-2 2h-4v-8h4"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    />
                </svg>
                <span class="button-label">Need help?</span>
            </button>
        </div>

        <script src="/components/need_help.js" />
    }
}
