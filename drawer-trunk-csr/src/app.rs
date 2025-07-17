use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 p-4">
            <h1 class="text-2xl font-bold">"Rust UI Starters — Trunk Tailwind"</h1>
            <p>"Starter template built with Leptos, Trunk and Tailwind CSS."</p>
            <DemoDrawer />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn DemoDrawer() -> impl IntoView {
    view! {
        <div class="grid place-items-center mainDiv min-h-[100svh]">

            <div class="grid content-center place-items-center w-full mainDivDrawer h-[100svh] bg-canvas">
                <h1 class="fluid">Drawer.</h1>
                <button
                    class="p-4 rounded-lg bg-neutral-500"
                    popovertargetaction="toggle"
                    popovertarget="drawer"
                >
                    Open Drawer
                </button>

            </div>
            <aside
                class="inset-0 p-0 m-0 w-auto h-auto bg-transparent border-0 drawer [&::backdrop]:bg-black/50"
                popover="auto"
                id="drawer"
            >
                <div class="flex overflow-y-auto overscroll-none flex-col items-center w-full h-full drawer__scroller [&::after]:order-[-999999] [&::after]:flex-[1_0_100svh] [&::after]:w-full [&::after]:h-[100svh]">
                    <div class="relative w-full drawer__slide max-w-[600px] max-h-[95%] flex-[1_0_600px] [&::after]:absolute [&::after]:inset-0 h-[600px]">
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
                        <div class="flex absolute flex-col justify-between w-full h-full border border-b-0 drawer__content z-[2] rounded-[12px_12px_0_0] text-[color:var(--color)] border-[color:color-mix(in_hsl,_canvas,_canvasText_10%)] bg-[canvas] pt-[44px]">
                            <button
                                autofocus
                                class="flex absolute top-0 flex-col justify-evenly items-center w-full text-sm border-0 border-b outline-none drawer__drag bg-[#0000] border-[color:color-mix(in_hsl,_canvas,_canvasText_10%)] h-[44px]"
                                popovertargetaction="hide"
                                popovertarget="drawer"
                            >
                                <span class="w-[8ch] h-[6px] rounded-[10px]"></span>
                                <span>Just a Drawer</span>
                            </button>
                            <div class="flex overflow-auto flex-col flex-1 gap-4 p-4 content">
                                <p>
                                    a Drawer demo made with modern web platform features as
                                    requested by the community.
                                </p>
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
