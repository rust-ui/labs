use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoDrawer() -> impl IntoView {
    view! {
        <Stylesheet id="drawer" href="/components/drawer.css" />

        <div class="mainBody min-h-[100svh] grid place-items-center text-[var(--color)]">

            <div class="mainDivDrawer grid place-items-center w-full content-center h-[100svh]">
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
                class="drawer bg-transparent p-0 m-0 border-0   [&::backdrop]:bg-black/50"
                popover="auto"
                id="drawer"
            >
                <div class="flex flex-col items-center w-full h-full overflow-y-auto drawer__scroller overscroll-none">
                    <div class="drawer__slide max-w-[600px] w-full relative max-h-[95%]">
                        <div class="absolute inset-0 z-[10] flex flex-col justify-between pointer-events-none drawer__anchors">
                            <div class="drawer__anchor h-[50px] w-full snap-end"></div>
                            <div class="drawer__anchor h-[50px] w-full snap-end"></div>
                        </div>
                        // <!-- Acts like a backdrop button so you don't body click -->
                        <button
                            popovertargetaction="hide"
                            popovertarget="drawer"
                            class="drawer__curtain absolute w-full opacity-0 bottom-0 border-0 left-1/2 translate-x-[-50%] translate-y-0 h-[100svh]"
                            tabindex="-1"
                        ></button>
                        <div class="drawer__content w-full h-full absolute z-[2] flex flex-col justify-between">
                            <button
                                autofocus
                                class="drawer__drag flex flex-col items-center justify-evenly bg-[#0000] text-sm absolute top-0 w-full outline-none border-b border-[var(--border)]"
                                popovertargetaction="hide"
                                popovertarget="drawer"
                            >
                                <span></span>
                                <span>"Just a drawer"</span>
                            </button>
                            <div class="flex flex-col flex-1 gap-4 p-4 overflow-auto content">
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
