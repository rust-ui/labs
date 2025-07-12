use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {ContainerFlexTransition, div, "container-flex-transition", "flex justify-between w-[500px] gap-3"}
    clx! {TransitionItem, div, "item", "flex justify-center items-center w-[80px] h-[80px] border border-red-500 font-bold text-red-500 bg-white box-border"}
}

pub use components::*;

use crate::components::ui::button::Button;

#[component]
pub fn DemoFlexTransition() -> impl IntoView {
    view! {
        <Stylesheet href="/components/flex_transition.css" />

        <fieldset class="flex flex-col border-none w-fit">
            <legend>Property</legend>
            <label>
                <input type="radio" name="property" value="flex-direction" checked />
                flex-direction
            </label>
            <label>
                <input type="radio" name="property" value="justify-content" />
                justify-content
            </label>
            <label>
                <input type="checkbox" id="delay" />
                Add delay
            </label>
        </fieldset>

        <Button class="my-2">switch</Button>

        <ContainerFlexTransition>
            <TransitionItem style="view-transition-name: item1">1</TransitionItem>
            <TransitionItem style="view-transition-name: item2">2</TransitionItem>
            <TransitionItem style="view-transition-name: item3">3</TransitionItem>
            <TransitionItem style="view-transition-name: item4">4</TransitionItem>
            <TransitionItem style="view-transition-name: item5">5</TransitionItem>
        </ContainerFlexTransition>

        <script src="/components/flex_transition.js" />
    }
}
