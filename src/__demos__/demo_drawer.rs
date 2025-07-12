use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoDrawer() -> impl IntoView {
    view! {
        <Stylesheet id="drawer" href="/components/drawer.css" />

        <div class="grid place-items-center mainBody min-h-[100svh] text-[var(--color)]">

            <div class="grid content-center place-items-center w-full mainDivDrawer h-[100svh]">
                <h1 class="fluid">Drawer.</h1>
                <p>requested by the community. built for the community.</p>
                <p class="text-green-500">Coucou</p>
                <button
                    class="p-4 rounded-lg bg-neutral-100"
                    popovertargetaction="toggle"
                    popovertarget="drawer"
                >
                    slide open
                </button>

            </div>
            <aside
                class="p-0 m-0 bg-transparent border-0 drawer [&::backdrop]:bg-black/50"
                popover="auto"
                id="drawer"
            >
                <div class="flex overflow-y-auto overscroll-none flex-col items-center w-full h-full drawer__scroller">
                    <div class="relative w-full drawer__slide max-w-[600px] max-h-[95%]">
                        <div class="flex absolute inset-0 flex-col justify-between pointer-events-none z-[10] drawer__anchors">
                            <div class="w-full drawer__anchor h-[50px] snap-end"></div>
                            <div class="w-full drawer__anchor h-[50px] snap-end"></div>
                        </div>
                        // <!-- Acts like a backdrop button so you don't body click -->
                        <button
                            popovertargetaction="hide"
                            popovertarget="drawer"
                            class="absolute bottom-0 left-1/2 w-full border-0 opacity-0 translate-y-0 drawer__curtain translate-x-[-50%] h-[100svh]"
                            tabindex="-1"
                        ></button>
                        <div class="flex absolute flex-col justify-between w-full h-full drawer__content z-[2]">
                            <button
                                autofocus
                                class="flex absolute top-0 flex-col justify-evenly items-center w-full text-sm border-b outline-none drawer__drag bg-[#0000] border-[var(--border)]"
                                popovertargetaction="hide"
                                popovertarget="drawer"
                            >
                                <span></span>
                                <span>"Just a drawer"</span>
                            </button>
                            <div class="flex overflow-auto flex-col flex-1 gap-4 p-4 content">
                                <p>
                                    a Drawer demo made with modern web platform features as
                                    requested by the community.
                                </p>
                                // <DemoButton />
                                <a href="/">
                                    <span>HOME</span>
                                </a>
                                <ul>
                                    <li>Popover API</li>
                                    <li>onscrollsnapchange</li>
                                    <li>CSS scroll snap</li>
                                    <li>CSS scroll-driven animations</li>
                                    <li>CSS @starting-style</li>
                                    <li>interactive-widget=resizes content</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
                <canvas
                    id="reaction-canvas"
                    class="fixed inset-0 w-full h-full pointer-events-none z-[999999]"
                ></canvas>
            </aside>

        </div>

        <script src="/components/drawer.js" />
    }
}
