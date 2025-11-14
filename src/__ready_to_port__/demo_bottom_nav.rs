use icons::{CircleUser, House, SlidersHorizontal, Wallet};
use leptos::prelude::*;
use leptos_ui::clx;
use strum::{Display, EnumIter, IntoEnumIterator};

mod components {
    use super::*;
    clx! {BottomNav, nav, "mx-auto w-full max-w-lg h-16 bg-card border-t border-border"}
    clx! {BottomNavGrid, div, "grid grid-cols-4 h-full font-medium"}
    clx! {BottomNavLink, button, "inline-flex flex-col justify-center items-center px-5 group [&_svg]:mb-2 [&_svg]:text-muted-foreground aria-[current=page]:[&_svg]:text-primary"}
    clx! {BottomNavLabel, span, "text-sm text-muted-foreground group-aria-[current=page]:text-primary"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, Default)]
enum NavPage {
    Home,
    #[default]
    Wallet,
    Settings,
    Profile,
}

impl NavPage {
    fn icon(self) -> impl IntoView {
        match self {
            NavPage::Home => view! { <House class="size-5" /> }.into_any(),
            NavPage::Wallet => view! { <Wallet class="size-5" /> }.into_any(),
            NavPage::Settings => view! { <SlidersHorizontal class="size-5" /> }.into_any(),
            NavPage::Profile => view! { <CircleUser class="size-5" /> }.into_any(),
        }
    }
}

#[component]
pub fn DemoBottomNav() -> impl IntoView {
    let active_page_signal = RwSignal::new(NavPage::Wallet);

    view! {
        <div class="flex flex-col my-10 rounded-t-2xl border h-[300px] w-[400px]">
            <div class="flex-1 bg-gray-200 rounded-t-2xl"></div>

            <BottomNav>
                <BottomNavGrid>
                    {NavPage::iter()
                        .map(|page| {
                            view! {
                                <BottomNavLink
                                    on:click=move |_| active_page_signal.set(page)
                                    attr:aria-current=move || {
                                        if active_page_signal.get() == page { "page" } else { "" }
                                    }
                                >

                                    {page.icon()}
                                    <BottomNavLabel>{page.to_string()}</BottomNavLabel>
                                </BottomNavLink>
                            }
                        })
                        .collect_view()}
                </BottomNavGrid>
            </BottomNav>
        </div>
    }
}
