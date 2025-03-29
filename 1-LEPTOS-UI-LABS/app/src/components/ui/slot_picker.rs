use leptos::prelude::*;
use leptos_ui::{clx, transition};

use crate::utils::utils::Utils;

mod components {
    use super::*;
    clx! {SlotDaysWrapper, div, "days"}
    clx! {SlotTitle, span, "day-title"}
    transition! {SlotDay, div, "day"}
    transition! {DayContainer, div, "day-container"}
    transition! {ButtonAdd, button, "add-btn"}
    transition! {TimeSlots, div, "time-slots"}
}

pub use components::*;
