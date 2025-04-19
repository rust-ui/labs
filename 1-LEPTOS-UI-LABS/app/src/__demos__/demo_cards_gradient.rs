use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::{a, clx};

// Type for pricing tier data
#[derive(Clone)]
struct PricingTier {
    name: &'static str,
    price: &'static str,
    hue: &'static str,
    features: Vec<&'static str>,
    cta_text: &'static str,
    cta_href: &'static str,
}

#[component]
pub fn DemoCardsGradient() -> impl IntoView {
    // Define pricing tiers
    let pricing_tiers = vec![
        PricingTier {
            name: "Basic",
            price: "$9.99",
            hue: "--hue: 165; --saturation: 82.26%; --lightness: 51.37%",
            features: vec![
                "Access to standard workouts and nutrition plans",
                "Email support",
            ],
            cta_text: "Get Started",
            cta_href: "#basic",
        },
        PricingTier {
            name: "Pro",
            price: "$19.99",
            hue: "--hue: 291.34; --saturation: 95.9%; --lightness: 61.76%",
            features: vec![
                "Access to advanced workouts and nutrition plans",
                "Priority Email support",
                "Exclusive access to live Q&A sessions",
            ],
            cta_text: "Upgrade to Pro",
            cta_href: "#pro",
        },
        PricingTier {
            name: "Ultimate",
            price: "$29.99",
            hue: "--hue: 338.69; --saturation: 100%; --lightness: 48.04%",
            features: vec![
                "Access to all premium workouts and nutrition plans",
                "24/7 Priority support",
                "1-on-1 virtual coaching session every month",
                "Exclusive content and early access to new features",
            ],
            cta_text: "Go Ultimate",
            cta_href: "#ultimate",
        },
    ];

    view! {
        <Stylesheet href="/components/cards_gradient.css" />

        <div class="grid place-items-center min-h-screen bg-[#212121] text-[#ddd] font-['League_Spartan']">
            <Main>
                <MainHeading>"Pricing"</MainHeading>
                <Cards>
                    {pricing_tiers
                        .into_iter()
                        .map(|tier| {
                            view! {
                                <Card style=tier.hue>
                                    <CardHeading>{tier.name}</CardHeading>
                                    <CardPrice>{tier.price}</CardPrice>
                                    <CardBullets role="list">
                                        {tier
                                            .features
                                            .into_iter()
                                            .map(|feature| {
                                                view! { <li>{feature}</li> }
                                            })
                                            .collect_view()}
                                    </CardBullets>
                                    <CardCta href=tier.cta_href>{tier.cta_text}</CardCta>
                                </Card>
                            }
                        })
                        .collect_view()}
                </Cards>
            </Main>
        </div>

        // * ----- SCRIPT -----
        <script src="/components/cards_gradient.js"></script>
    }
}

mod components {
    use super::*;

    clx! {Main, div, "main max-w-[75rem] p-[3em_1.5em]"}
    clx! {MainHeading, h1, "text-[2.25em] font-semibold mb-[0.75em] text-center text-[#eceff1]"}
    clx! {Cards, div, "flex flex-wrap gap-10"}
    clx! {Card, div, "card flex-1 basis-[14rem] p-[1.5em_2em] grid grid-rows-[auto_auto_auto_1fr] items-start gap-5 text-[#eceff1] bg-[#2b2b2b] rounded-[15px] relative overflow-hidden border border-[#2b2b2b]"}
    clx! {CardHeading, h2, "text-[1.05em] font-semibold"}
    clx! {CardPrice, p, "text-[1.75em] font-bold"}
    clx! {CardBullets, ul, "card__bullets flow list-none leading-[1.4]"}

    a! {CardCta, "cta", "block self-end mt-4 mb-2 text-center no-underline p-[0.7em] rounded-[10px] text-base font-semibold"}
}

pub use components::*;
