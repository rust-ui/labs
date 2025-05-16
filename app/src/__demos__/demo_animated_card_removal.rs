use leptos::prelude::*;
use leptos_ui::utils::Utils;
use leptos_ui::{clx, transition};

#[component]
fn DeleteButton() -> impl IntoView {
    view! {
        <DeleteButtonWrapper class="delete-btn">
            <img
                src="https://upload.wikimedia.org/wikipedia/commons/7/7d/Trash_font_awesome.svg"
                alt="close button"
                class="color-white"
            />
            <span class="border-0 clip-[rect(1px,1px,1px,1px)] clip-path-[inset(50%)] h-px -m-px overflow-hidden p-0 absolute w-px whitespace-nowrap">
                Delete
            </span>
        </DeleteButtonWrapper>
    }
}

struct ItemData {
    color: String,
}

#[component]
pub fn DemoAnimatedCardRemoval() -> impl IntoView {
    let items = vec![
        ItemData {
            color: "bg-[tan]".to_string(),
        },
        ItemData {
            color: "bg-[khaki]".to_string(),
        },
        ItemData {
            color: "bg-[thistle]".to_string(),
        },
        ItemData {
            color: "bg-[lightblue]".to_string(),
        },
        ItemData {
            color: "bg-[lightgreen]".to_string(),
        },
    ];

    view! {
        <Container>
            {
                items.into_iter().map(|item| {
                    view! {
                        <Item class=format!("{}", item.color)>
                            <DeleteButton />
                        </Item>
                    }
                }).collect::<Vec<_>>()
            }
        </Container>

        <script src="/components/cards_removal.js"></script>
    }
}

mod components {
    use super::*;

    clx! {Container, div, "flex justify-center w-full gap-8 max-w-[1000px]"}
    transition! {Item, li, "w-full aspect-[2/3] block relative rounded-xl max-w-[220px]"}
    clx! {DeleteButtonWrapper, button, "absolute -bottom-3 -right-3 w-12 h-12 p-2 border-4 rounded-full bg-[steelblue] text-white"}
}

pub use components::*;
