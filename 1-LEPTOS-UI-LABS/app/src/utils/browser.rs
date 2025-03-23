use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ JAVASCRIPT ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// const showHideBtn = document.querySelector('.show-hide-btn');
//     const container = document.querySelector('.container');

//     showHideBtn.addEventListener('click', () => {
//       document.startViewTransition(() => {
//         container.classList.toggle('expanded');
//       });
//     });

pub fn get_document() -> Option<web_sys::Document> {
    window()?.document()
}

pub fn start_view_transition<F>(callback: F)
where
    F: FnOnce() + 'static,
{
    // Only attempt to use the API in the browser
    if !cfg!(target_arch = "wasm32") {
        callback();
        return;
    }

    if let Some(document) = get_document() {
        let document_js_value = document.dyn_into::<JsValue>().unwrap();

        let start_transition_fn = Reflect::get(
            &document_js_value,
            &JsValue::from_str("startViewTransition"),
        )
        .unwrap_or(JsValue::UNDEFINED);

        if !start_transition_fn.is_undefined() {
            let callback_js_closure = Closure::once(Box::new(callback) as Box<dyn FnOnce()>);

            let callback_js_value = callback_js_closure.as_ref().clone();

            let _ = js_sys::Reflect::apply(
                &start_transition_fn.dyn_into().unwrap(),
                &document_js_value,
                &js_sys::Array::of1(&callback_js_value),
            );

            // Prevent memory leaks
            callback_js_closure.forget();
        } else {
            // * Fallback if the API is not available - just run the callback
            callback();
        }
    } else {
        // * Fallback if document is not available
        callback();
    }
}
