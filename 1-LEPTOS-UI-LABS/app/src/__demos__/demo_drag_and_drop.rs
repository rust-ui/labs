use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Main, div, "m-0"}
    clx! {Container, div, "container bg-[#333] p-4 mt-4"}
    clx! {SideLinks, div, "absolute top-[15px] right-[15px]"}
}

pub use components::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn SideLink(
    href: &'static str,
    icon_class: &'static str,
    label: &'static str,
    additional_class: &'static str,
) -> impl IntoView {
    let class = format!(
        "flex items-center justify-center no-underline mb-[10px] text-white w-[180px] py-[10px] rounded-[10px] {} {}",
        additional_class, icon_class
    );

    view! {
        <a href=href target="_blank" class=class>
            <i class=icon_class></i>
            <span class="ml-[10px] text-lg">{label}</span>
        </a>
    }
}

#[component]
pub fn DraggableItem(text: String) -> impl IntoView {
    view! {
        <p class="draggable p-4 bg-white border border-black cursor-move" draggable="true">{text}</p>
    }
}
#[component]
pub fn DemoDragAndDrop() -> impl IntoView {
    view! {
        <Stylesheet href="/components/drag_and_drop.css" />
        <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
        <div class="relative gap-8">
            <Main>
                <Container>
                    <DraggableItem text="1".into() />
                    <DraggableItem text="2".into() />
                </Container>
                <Container>
                    <DraggableItem text="3".into() />
                    <DraggableItem text="4".into() />
                </Container>

            </Main>
            <SideLinks>
                <SideLink
                    href="https://youtu.be/jfYWwQrtzzY"
                    icon_class="fab fa-youtube-square"
                    label="View Tutorial"
                    additional_class="bg-red-700"
                />
                <SideLink
                    href="https://github.com/WebDevSimplified"
                    icon_class="fab fa-github-square"
                    label="View GitHub"
                    additional_class="bg-[#6e5494] text-white text-3xl"
                />
                <SideLink
                    href="https://twitter.com/DevSimplified"
                    icon_class="fab fa-twitter-square"
                    label="View Twitter"
                    additional_class="bg-[#1DA1F2]"
                />
            </SideLinks>
        </div>
        <script src="/components/drag_and_drop.js"></script>
    }
}
