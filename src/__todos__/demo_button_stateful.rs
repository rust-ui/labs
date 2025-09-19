use leptos::prelude::*;
use crate::components::ui::button::Button;
use crate::utils::hooks::use_view_transition::use_view_transition;

// Button state enum
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ButtonState {
    Idle,
    Working,
    Done,
}

impl ButtonState {
    pub fn text(&self) -> &'static str {
        match self {
            ButtonState::Idle => "Do some hard work",
            ButtonState::Working => "⏳ working...",
            ButtonState::Done => "Done! ✅",
        }
    }
}

#[component]
pub fn DemoButtonStateful() -> impl IntoView {
    let button_state = RwSignal::new(ButtonState::Idle);
    let transition = use_view_transition();

    let handle_click = move |_| {
        if button_state.get() == ButtonState::Idle {
            // Start sequence: Idle -> Working
            transition(Box::new(move || button_state.set(ButtonState::Working)));

            // Working -> Done (after 2s)
            let transition_2 = transition.clone();
            set_timeout(
                move || transition_2(Box::new(move || button_state.set(ButtonState::Done))),
                std::time::Duration::from_millis(2000),
            );

            // Done -> Idle (after 4s)
            let transition_3 = transition.clone();
            set_timeout(
                move || transition_3(Box::new(move || button_state.set(ButtonState::Idle))),
                std::time::Duration::from_millis(4000),
            );
        }
    };

    view! {
        <style>
            {r#"
            @media (prefers-reduced-motion: no-preference) {
            ::view-transition-old(button-stateful),
            ::view-transition-new(button-stateful) {
            height: 100%;
            width: 100%;
            }
            }
            "#}
        </style>

        <Button
            id="ButtonStateful"
            style="view-transition-name: button-stateful;"
            on:click=handle_click
        >
            {move || button_state.get().text()}
        </Button>
    }
}
