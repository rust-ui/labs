use leptos::prelude::*;
use leptos_ui::{clx, div, transition};

use crate::utils::utils::Utils;

mod components {
    use super::*;
    clx! {SlotDaysWrapper, div, "days"}
    clx! {SlotTitle, span, "day-title"}
    transition! {SlotDay, div, "day"}
    transition! {DayContainer, div, "day-container"}
    transition! {ButtonAdd, button, "add-btn"}
    div! {RootTimeSlots, "time-slots"}
}

pub use components::*;

#[component]
pub fn TimeSlots() -> impl IntoView {
    let random_name = Utils::use_random_transition_name();

    view! { <RootTimeSlots style=random_name /> }
}
