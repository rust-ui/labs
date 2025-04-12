use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::clx;


#[component]
pub fn DemoJsAlertDialog() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    let show_dialog = move |_| {
        set_is_open(true);
    };
    let hide_dialog = move |_| {
        set_is_open(false);
    };

    let open_attribute = move |_|{
        if is_open.get() {
            "flex"
        } else {
            "none"
        }
    };

    view! {
       <div class="flex justify-center items-center h-screen">
        <button on:click=show_dialog class="bg-blue-500 text-white px-4 py-2 rounded-md">
            "Open Dialog"
        </button>
        <div class="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center" style=clx!("display: {open_attribute}")>
            <div class="bg-white p-6 rounded-md">
                <h1 class="text-2xl font-bold">Alert Dialog</h1>
                <p class="text-gray-500">This is an alert dialog</p>
                <button on:click=hide_dialog class="bg-blue-500 text-white px-4 py-2 rounded-md">
                    "Close Dialog"
                </button>
            </div>
        </div>
       </div>
    }
}
