use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {DayContainer, div, "day-container"}
    clx! {ButtonAdd, button, "add-btn"}
}

pub use components::*;
