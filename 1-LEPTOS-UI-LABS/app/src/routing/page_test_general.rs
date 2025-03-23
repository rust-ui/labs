use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

#[component]
pub fn PageTestGeneral() -> impl IntoView {
    view! {
        <div>
            <h1>Page Test General</h1>
            <DemoButtonWithClx />
            <DemoTest />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn DemoButtonWithClx() -> impl IntoView {
    clx! {MyButton, button, "px-4 py-2 bg-blue-500 text-white rounded"}

    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <div>
            <MyButton on:click=on_click>"Click Me: " {count}</MyButton>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn DemoTest() -> impl IntoView {
    view! {
        <div class="flex overflow-hidden relative justify-center items-center p-20 w-full h-full rounded-lg border md:shadow-xl min-h-[300px] bg-background">
            <p class="z-10 text-5xl font-medium tracking-tighter text-center whitespace-pre-wrap">
                Retro
            </p>
            <BackgroundRetro>
                <BackgroundRetroGrid />
                <div class="absolute inset-0 bg-gradient-to-t from-white to-transparent dark:from-black to-90%" />
            </BackgroundRetro>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

mod components {
    use super::*;
    clx! {BackgroundRetro, div, "absolute w-full h-full opacity-50 [perspective:200px]"}
}

pub use components::*;

use crate::components::ui::_animations::ANIMATIONS;

#[component]
pub fn BackgroundRetroGrid(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    let class = Memo::new(move |_| {
        tw_merge!(
            ANIMATIONS.BACKGROUND_RETRO_GRID,
            "[background-image:linear-gradient(to_right,rgba(0,0,0,0.3)_1px,transparent_0),linear-gradient(to_bottom,rgba(0,0,0,0.3)_1px,transparent_0)] [background-repeat:repeat] [background-size:60px_60px] [height:300vh] [inset:0%_0px] [margin-left:-50%] [transform-origin:100%_0_0] [width:600vw] dark:[background-image:linear-gradient(to_right,rgba(255,255,255,0.2)_1px,transparent_0),linear-gradient(to_bottom,rgba(255,255,255,0.2)_1px,transparent_0)]",
            class()
        )
    });

    view! {
        <div class="absolute inset-0 [transform:rotateX(35deg)]">
            <div class=class />
        </div>
    }
}
