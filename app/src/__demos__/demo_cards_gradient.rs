use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::{a, clx};

#[component]
pub fn DemoCardsGradient() -> impl IntoView {
    view! {
        <Stylesheet href="/components/cards_gradient.css" />

        <div class="grid place-items-center min-h-screen bg-[#212121] text-[#ddd] font-['League_Spartan']">
            <div class="main max-w-[75rem] p-[3em_1.5em]">
                <Heading>"Pricing"</Heading>
                <Cards>
                    <DemoCard style="--hue: 165; --saturation: 82.26%; --lightness: 51.37%"/>
                    <DemoCard style="--hue: 291.34; --saturation: 95.9%; --lightness: 61.76%"/>
                    <DemoCard style="--hue: 338.69; --saturation: 100%; --lightness: 48.04%"/>
                </Cards>
            </div>
        </div>

        // * ----- SCRIPT -----
        <script src="/components/cards_gradient.js"></script>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn DemoCard(#[prop(into)] style: &'static str) -> impl IntoView {
    view! {
        <Card style=style>
            <CardHeading>"Plan"</CardHeading>
            <CardPrice>"$29.99"</CardPrice>
            <CardBullets role="list">
                <li>"Lorem ipsum dolor sit amet"</li>
                <li>"Consectetur adipiscing elit"</li>
                <li>"Sed do eiusmod tempor incididunt"</li>
            </CardBullets>
            <CardCta href="#ultimate">"My CTA"</CardCta>
        </Card>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

mod components {
    use super::*;

    clx! {Heading, h1, "text-[2.25em] font-semibold mb-[0.75em] text-center text-[#eceff1]"}
    clx! {Cards, div, "flex flex-wrap gap-10"}
    clx! {Card, div, "card flex-1 basis-[14rem] p-[1.5em_2em] grid grid-rows-[auto_auto_auto_1fr] items-start gap-5 text-[#eceff1] bg-[#2b2b2b] rounded-[15px] relative overflow-hidden border border-[#2b2b2b]"}
    clx! {CardHeading, h2, "text-[1.05em] font-semibold"}
    clx! {CardPrice, p, "text-[1.75em] font-bold"}
    clx! {CardBullets, ul, "card__bullets flow list-none leading-[1.4]"}

    a! {CardCta, "cta", "block self-end mt-4 mb-2 text-center no-underline p-[0.7em] rounded-[10px] text-base font-semibold"}
}

pub use components::*;
