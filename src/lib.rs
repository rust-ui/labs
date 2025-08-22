#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]

#[allow(non_snake_case)]
pub mod __TODOS__;
pub mod __ready_to_port__;
pub mod components;
pub mod gsap_intro_page;
pub mod page_test;
pub mod shared;

pub mod app;
pub mod shell;

#[cfg(feature = "ssr")]
pub mod fileserv;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
