use leptos::prelude::*;

use crate::components::demos::demo_js_payment_method::DemoJsPaymentMethod;

#[component]
pub fn PagePaymentMethod() -> impl IntoView {
    view! { <DemoJsPaymentMethod /> }
}
