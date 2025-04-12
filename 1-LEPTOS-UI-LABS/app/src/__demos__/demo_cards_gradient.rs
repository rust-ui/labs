use leptos::prelude::*;
use leptos_ui::clx;

// JavaScript
const CARDS_GRADIENT_JS: &str = r#"
(function() {
    const applyMouseTracking = () => {
        const cards = document.querySelectorAll('.card');
        cards.forEach(card => {
            card.addEventListener('mousemove', (e) => {
                const rect = card.getBoundingClientRect();
                const x = ((e.clientX - rect.left) / rect.width) * 100;
                const y = ((e.clientY - rect.top) / rect.height) * 100;
                card.style.setProperty('--x', `${x}%`);
                card.style.setProperty('--y', `${y}%`);
            });
        });
    };
    
    // For SSR, wait until content is loaded
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', applyMouseTracking);
    } else {
        applyMouseTracking();
    }
    
    // For CSR/hydration, run again after Leptos has hydrated
    if (window._$LEPTOS) {
        window._$LEPTOS.on('leptos:end', applyMouseTracking);
    }
})();
"#;

// CSS 
const PRICING_CARDS_CSS: &str = r#"
/* Only essential custom CSS that can't be done with Tailwind */
@import url("https://fonts.googleapis.com/css2?family=League+Spartan:wght@400;500;600;700;800;900&display=swap");

/* Card base styles */
.card {
  --hsl: var(--hue), var(--saturation), var(--lightness);
  --x: 50%;
  --y: 50%;
  position: relative;
  isolation: isolate;
  overflow: hidden;
  border: 1px solid rgba(236, 239, 241, 0.2);
}

/* Gradient hover effect */
.card::before {
  content: "";
  position: absolute;
  inset: 0;
  background: radial-gradient(
    circle at var(--x) var(--y),
    hsla(var(--hsl), 0.25) 0%,
    transparent 65%
  );
  opacity: 0;
  transition: opacity 0.1s linear;
  z-index: -1;
}

.card:hover {
  border-color: hsla(var(--hsl), 0.3);
  box-shadow: 0 0 0 1px hsla(var(--hsl), 0.2);
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
}

.card:hover::before {
  opacity: 1;
}

/* Checkbox item styling */
.card__bullets li {
  padding-left: 1.5rem;
  position: relative;
}

.card__bullets li::before {
  content: "";
  position: absolute;
  left: 0;
  top: 0.25em;
  width: 1rem;
  height: 1rem;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 512 512' width='16' title='check' fill='%23dddddd'%3E%3Cpath d='M173.898 439.404l-166.4-166.4c-9.997-9.997-9.997-26.206 0-36.204l36.203-36.204c9.997-9.998 26.207-9.998 36.204 0L192 312.69 432.095 72.596c9.997-9.997 26.207-9.997 36.204 0l36.203 36.204c9.997 9.997 9.997 26.206 0 36.204l-294.4 294.401c-9.998 9.997-26.207 9.997-36.204-.001z' /%3E%3C/svg%3E");
  background-size: contain;
  background-repeat: no-repeat;
}

/* CTA button styling */
.cta {
  background-color: #0d0d0d;
  color: white;
  transition: background-color 0.2s ease;
}

.cta:hover {
  background-color: hsl(var(--hsl));
}

/* Remove automatic button glow when hovering card */
.card:hover .cta {
  background-color: #0d0d0d; /* Keep default until directly hovered */
}

/* Ensure direct hover takes precedence */
.card:hover .cta:hover {
  background-color: hsl(var(--hsl));
}

/* Flow utility for spacing */
.flow > * + * {
  margin-top: 0.5em;
}
"#;

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
        <div class="grid place-items-center min-h-screen bg-[#212121] text-[#ddd] font-['League_Spartan']">
            <Main>
                <MainHeading>"Pricing"</MainHeading>
                <Cards>
                    {pricing_tiers.into_iter().map(|tier| {
                        view! {
                            <Card style=tier.hue>
                                <CardHeading>{tier.name}</CardHeading>
                                <CardPrice>{tier.price}</CardPrice>
                                <CardBullets role="list">
                                    {tier.features.into_iter().map(|feature| {
                                        view! { <li>{feature}</li> }
                                    }).collect_view()}
                                </CardBullets>
                                <CardCta href=tier.cta_href>
                                    {tier.cta_text}
                                </CardCta>
                            </Card>
                        }
                    }).collect_view()}
                </Cards>
            </Main>
        </div>

        // * ----- STYLES -----
        <style>
            {PRICING_CARDS_CSS}
        </style>

        // * ----- SCRIPT -----
        <script>
            {CARDS_GRADIENT_JS}
        </script>
    }
}

mod components {
    use super::*;

    clx! {Main, div, "main max-w-[75rem] p-[3em_1.5em]"}
    clx! {MainHeading, h1, "text-[2.25em] font-semibold mb-[0.75em] text-center text-[#eceff1]"}
    clx! {Cards, div, "flex flex-wrap gap-10"}
    clx! {Card, div, "card flex-1 basis-[14rem] p-[1.5em_2em] grid grid-rows-[auto_auto_auto_1fr] items-start gap-5 text-[#eceff1] bg-[#2b2b2b] rounded-[15px]"}
    clx! {CardHeading, h2, "text-[1.05em] font-semibold"}
    clx! {CardPrice, p, "text-[1.75em] font-bold"}
    clx! {CardBullets, ul, "card__bullets flow list-none leading-[1.4]"}

    // Custom component for CardCta with href attribute
    #[component]
    pub fn CardCta(
        #[prop(into, optional)] class: Signal<String>,
        #[prop(optional)] href: &'static str,
        children: Children,
    ) -> impl IntoView {
        let merged_classes = Memo::new(move |_| {
            tw_merge::tw_merge!(
                "cta block self-end mt-4 mb-2 text-center no-underline p-[0.7em] rounded-[10px] text-base font-semibold",
                class()
            )
        });

        view! {
            <a class=merged_classes href=href>
                {children()}
            </a>
        }
    }
}

pub use components::*;
