use leptos::prelude::*;
use leptos_ui::{clx, div};

use crate::utils::utils::Utils;

mod components {
    use super::*;
    clx! {SlotDays, div, "days"}
    clx! {SlotTitle, span, "day-title"}
    clx! {RootDay, div, "day-container"}
    clx! {RootAddBtn, button, "add-btn"}
    div! {RootTimeSlots, "time-slots"}
}

pub use components::*;

#[component]
pub fn DayContainer(children: Children) -> impl IntoView {
    let random_name = Utils::use_random_transition_name();

    view! { <RootDay style=random_name>{children()}</RootDay> }
}

#[component]
pub fn ButtonAdd(children: Children) -> impl IntoView {
    let random_name = Utils::use_random_transition_name();

    view! { <RootAddBtn style=random_name>{children()}</RootAddBtn> }
}

#[component]
pub fn TimeSlots() -> impl IntoView {
    let random_name = Utils::use_random_transition_name();

    view! { <RootTimeSlots style=random_name /> }
}
