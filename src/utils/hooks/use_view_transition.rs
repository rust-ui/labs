
/// Reusable view transition hook
pub fn use_view_transition() -> impl Fn(Box<dyn FnOnce()>) + Clone {
    move |callback: Box<dyn FnOnce()>| {
        if cfg!(feature = "ssr") {
            callback();
            return;
        }

        // Try view transition, fallback to immediate execution
        let script = r#"
            if (document.startViewTransition) {
                document.startViewTransition(() => { window._leptos_callback(); });
            } else {
                window._leptos_callback();
            }
        "#;

        // Set callback on window
        if let Some(window) = web_sys::window() {
            let closure = wasm_bindgen::closure::Closure::once_into_js(callback);
            let _ = js_sys::Reflect::set(&window, &wasm_bindgen::JsValue::from_str("_leptos_callback"), &closure);
            let _ = js_sys::eval(script);
        } else {
            callback();
        }
    }
}

/// Simple wrapper function for convenience
pub fn with_view_transition<F>(callback: F)
where
    F: FnOnce() + 'static,
{
    let transition = use_view_transition();
    transition(Box::new(callback));
}