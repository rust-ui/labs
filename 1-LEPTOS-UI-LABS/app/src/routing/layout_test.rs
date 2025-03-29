use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn LayoutTest() -> impl IntoView {
    view! {
        <div class="flex gap-4 p-2 mx-4">
            <div class="flex flex-col h-full gap-4 bg-neutral-500 w-[300px]">
                <a href="/test/general">"General"</a>
                <a href="/test/js-need-help">"JS Need Help"</a>
                <a href="/test/js-grid-collection">"JS Grid Collection"</a>
                <a href="/test/payment-method">"Payment Method"</a>
                <a href="/test/vercel-interaction">"Vercel Interaction"</a>
                <a href="/test/js-drawer">"JS Drawer"</a>
                <a href="/test/tailwind-tabs">"Tailwind Tabs"</a>
                <a href="/test/scroll-view-progress">"Scroll View Progress"</a>
                <a href="/test/scroll-top-button">"Scroll Top Button"</a>
                <a href="/test/demo-tailwind">"Demo Tailwind"</a>
            </div>

            <div class="w-full">
                <Outlet />
            </div>
        </div>
    }
}
