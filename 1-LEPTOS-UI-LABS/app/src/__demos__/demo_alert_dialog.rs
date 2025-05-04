use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Wrapper, div, "wrapper"}
    clx! {Button, button, "button"}
    clx! {NotifyWrapper, div, "notify-wrapper"}
    clx! {Notify, div, "notify"}
    clx! {NotifyContent, div, "notify-content"}
    clx! {NotifyHeader, div, "notify-header"}
    clx! {NotifyMain, div, "notify-main"}
    clx! {NotifyFooter, div, "notify-footer"}
    clx! {NotifyButtonConfirm, button, "notify-button notify-button__confirm"}
    clx! {NotifyButtonCancel, button, "notify-button notify-button__cancel"}
}

pub use components::*;

#[component]
pub fn DemoAlertDialog() -> impl IntoView {
    view! {
        <Stylesheet href="/components/alert_dialog.css" />
        
        <Wrapper>
            <Button attr:notify-toggler>
                "Notify me"
            </Button>
            
            <NotifyWrapper attr:notify-wrapper>
                <Notify>
                    <NotifyContent>
                        <NotifyHeader>"How to do"</NotifyHeader>
                        <NotifyMain>
                            "This is a notification"
                        </NotifyMain>
                    </NotifyContent>
                    <NotifyFooter>
                        <NotifyButtonConfirm>
                            "Ok"
                        </NotifyButtonConfirm>
                        <NotifyButtonCancel attr:notify-cancel>
                            "Cancel"
                        </NotifyButtonCancel>
                    </NotifyFooter>
                </Notify>
            </NotifyWrapper>
        </Wrapper>

        <script src="/components/alert_dialog.js"></script>
    }
}
