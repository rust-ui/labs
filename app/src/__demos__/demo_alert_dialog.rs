use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Wrapper, div, "w-full h-screen flex items-center justify-center relative overflow-hidden bg-[#1e1e1e]"}
    clx! {Button, button, "min-w-24 border-2 border-[#0a6ed1] rounded px-3 py-8 text-white bg-[#0a6ed1] font-medium cursor-pointer font-['Inter','sans-serif'] active:scale-95"}
    clx! {NotifyWrapper, div, "notify-wrapper fixed inset-0 bg-black/50 justify-center items-center z-50 hidden"}
    clx! {Notify, div, "notify w-full max-w-[388px] rounded-2xl bg-[rgba(34,34,34,0.9)] backdrop-blur-[18px] overflow-hidden transition-all duration-200 ease-in-out scale-80 opacity-0"}
    clx! {NotifyContent, div, "!py-8 !px-16 relative"}
    clx! {NotifyHeader, div, "text-center text-white text-lg font-semibold mb-3.5 font-['Inter',sans-serif]"}
    clx! {NotifyMain, div, "text-center text-[#929292] font-['Inter',sans-serif]"}
    clx! {NotifyFooter, div, "w-full flex h-16"}
    clx! {NotifyButtonConfirm, button, "flex-1 align-center justify-center text-decoration-none text-white font-['Inter',sans-serif] border-0 border-top border-[#3a3a3a] bg-transparent cursor-pointer hover:border-[rgba(10,110,209,0.9)] hover:bg-[rgba(10,110,209,0.9)] backdrop-filter-blur-18px"}
    clx! {NotifyButtonCancel, button, "flex-1 align-center justify-center text-decoration-none text-white font-['Inter',sans-serif] border-0 border-top border-[#3a3a3a] bg-transparent cursor-pointer hover:bg-[rgba(238,68,68,0.9)] hover:border-[rgba(238,68,68,0.9)]"}
}

pub use components::*;

#[component]
pub fn DemoAlertDialog() -> impl IntoView {
    view! {
        <Stylesheet href="/components/alert_dialog.css" />

        <Wrapper>
            <Button attr:notify-toggler>"Notify me"</Button>

            <NotifyWrapper attr:notify-wrapper>
                <Notify>
                    <NotifyContent>
                        <NotifyHeader>"How to do"</NotifyHeader>
                        <NotifyMain>"This is a notification"</NotifyMain>
                    </NotifyContent>
                    <NotifyFooter>
                        <NotifyButtonConfirm>"Ok"</NotifyButtonConfirm>
                        <NotifyButtonCancel attr:notify-cancel>"Cancel"</NotifyButtonCancel>
                    </NotifyFooter>
                </Notify>
            </NotifyWrapper>
        </Wrapper>

        <script src="/components/alert_dialog.js"></script>
    }
}
