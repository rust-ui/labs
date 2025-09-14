use icons::SvgIcon;
use leptos::prelude::*;

#[component]
pub fn TestPage() -> impl IntoView {
    view! {
        <div>
            <h1>Test Page</h1>
            <PlusAnimate />
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn PlusAnimate() -> impl IntoView {
    view! {
        <style>{PLUS_ANIMATION}</style>

        <SvgIcon data_name="PlusAnimate">
            <path d="M5 12h14" />
            <path d="M12 5v14" />
        </SvgIcon>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const PLUS_ANIMATION: &str = r#"
[data-name="PlusAnimate"] {
transition: transform 0.7s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}
[data-name="PlusAnimate"]:hover {
transform: rotate(180deg);
cursor: pointer;
}
"#;
