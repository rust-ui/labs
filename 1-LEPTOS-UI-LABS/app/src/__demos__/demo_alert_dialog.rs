use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoAlertDialog() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    
    let show_dialog = move |_| {
        set_is_open(true);
    };
    
    let hide_dialog = move |_| {
        set_is_open(false);
    };
    
    let confirm_dialog = move |_| {
        set_is_open(false);
    };

    view! {
        <Stylesheet href="/components/alert_dialog.css" />
        
        <div class="wrapper">
            <button class="button" type="button" on:click=show_dialog>
                "Notify me"
            </button>
            
            <div class=move || {
                let mut classes = "notify-wrapper".to_string();
                if is_open() {
                    classes += " active";
                }
                classes
            }>
                <div class="notify">
                    <div class="notify-content">
                        <div class="notify-header">"How to do"</div>
                        <div class="notify-main">
                            "This is a notification"
                        </div>
                    </div>
                    <div class="notify-footer">
                        <button class="notify-button notify-button__confirm" type="button" on:click=confirm_dialog>
                            "Ok"
                        </button>
                        <button class="notify-button notify-button__cancel" type="button" on:click=hide_dialog>
                            "Cancel"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
