#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::app::App;
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
