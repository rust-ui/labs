use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn LayoutHome() -> impl IntoView {
    view! {
        <div class="flex gap-4 p-2 mx-4">
            <div class="flex flex-col h-full gap-4 bg-neutral-500 w-[300px]">
                <a href="/general">"General"</a>
                <a href="/js-need-help">"JS Need Help"</a>
                <a href="/js-grid-collection">"JS Grid Collection"</a>
                <a href="/payment-method">"Payment Method"</a>
                <a href="/vercel-interaction">"Vercel Interaction"</a>
                <a href="/js-drawer">"JS Drawer"</a>
                <a href="/tailwind-tabs">"Tailwind Tabs"</a>
                <a href="/scroll-view-progress">"Scroll View Progress"</a>
                <a href="/scroll-top-button">"Scroll Top Button"</a>
            </div>

            <div class="w-full">
                <Outlet />
            </div>
        </div>
    }
}
