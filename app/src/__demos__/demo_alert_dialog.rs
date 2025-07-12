use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Wrapper, div, "w-full h-screen flex items-center justify-center relative overflow-hidden bg-[#1e1e1e] font-inter"}
    clx! {Button, button, "min-w-[96px] px-8 py-3 rounded border-2 border-blue-600 bg-blue-600 text-white font-medium active:scale-95 transition"}
    clx! {NotifyWrapper, div, "fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50"}
    clx! {Notify, div, "w-full max-w-[388px] bg-gray-800/90 backdrop-blur-[18px] rounded-2xl scale-100 opacity-100 transition-all duration-200"}
    clx! {NotifyContent, div, "px-16 py-8 text-center"}
    clx! {NotifyHeader, div, "text-white text-lg font-semibold mb-3"}
    clx! {NotifyMain, div, "text-[#929292]"}
    clx! {NotifyFooter, div, "w-full h-16 flex border-t border-gray-700"}
    clx! {NotifyButtonConfirm, button, "flex-1 uppercase text-white hover:bg-blue-600 hover:border-blue-600 transition backdrop-blur-[18px] border-r border-gray-700"}
    clx! {NotifyButtonCancel, button, "flex-1 text-white hover:bg-red-500 hover:border-red-500 transition backdrop-blur-[18px]"}
}

pub use components::*;

#[component]
pub fn DemoAlertDialog() -> impl IntoView {
    let notify_open = RwSignal::new(false);

    let on_ok = move |_| {
        notify_open.set(false);
    };

    let on_cancel = move |_| {
        notify_open.set(false);
    };

    view! {
        <Wrapper>
            <Button on:click=move |_| notify_open.set(true)>"Notify me"</Button>

            <Show when=move || notify_open.get()>
                <NotifyWrapper>
                    <Notify>
                        <NotifyContent>
                            <NotifyHeader>"How to do"</NotifyHeader>
                            <NotifyMain>"This is a notification"</NotifyMain>
                        </NotifyContent>
                        <NotifyFooter>
                            <NotifyButtonConfirm on:click=on_ok>"Ok"</NotifyButtonConfirm>
                            <NotifyButtonCancel on:click=on_cancel>"Cancel"</NotifyButtonCancel>
                        </NotifyFooter>
                    </Notify>
                </NotifyWrapper>
            </Show>
        </Wrapper>
    }
}
