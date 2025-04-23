use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn CustomCursorButton(
    text: String,
    cursor_text: String,
    #[prop(optional)] text_color: Option<String>,
    #[prop(optional)] bg_color: Option<String>,
    #[prop(optional)] href: Option<String>,
) -> impl IntoView {
    let style = format!(
        "--button-text-color: {}; --button-bg-color: {};",
        text_color
            .clone()
            .unwrap_or("var(--color-neutral-800)".into()),
        bg_color.clone().unwrap_or("var(--color-light)".into())
    );

    let content = view! {
        <p class="button-text">{text}</p>
        <div class="button-bg"></div>
    };

    if let Some(link) = href {
        view! {
            <a href=link data-cursor=cursor_text class="button" style=style>
                {content}
            </a>
        }
    } else {
        *Box::new(view! {
            <a href="#".to_string() data-cursor=cursor_text class="button" style=style>
                {content}
            </a>
        })
    }
}

#[component]
pub fn DemoCustomCursor() -> impl IntoView {
    view! {
        <Stylesheet href="/components/dynamic_cursor.css" />
        <div class="mainDiv">
            <section class="cloneable">
                <div class="button-row">
                    <CustomCursorButton
                        text="A GSAP-powered custom cursor".into()
                        cursor_text="Pretty cool, right?".into()
                    />

                    <CustomCursorButton
                        text="With dynamic cursor text".into()
                        cursor_text="This text comes from an attribute".into()
                        text_color="var(--color-neutral-200)".into()
                        bg_color="var(--color-neutral-700)".into()
                    />
                </div>

                <div class="cursor">
                    <p>Learn more</p>
                </div>
            </section>
        </div>
        <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
        <script src="/components/dynamic_cursor.js"></script>
    }
}
