use icons::{Beaker, House, Settings, TestTube};
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::__ready_to_port__::bottom_nav::{
    BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel,
};

#[component]
pub fn AppBottomNav() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();

    let is_active = move |path: &'static str| -> &'static str {
        if location.pathname.get() == path {
            "page"
        } else {
            ""
        }
    };

    view! {
        <BottomNav class="fixed right-0 bottom-0 left-0 z-50 sm:hidden !h-[calc(4rem+env(safe-area-inset-bottom))] bg-background">
            <BottomNavGrid class="!h-full !pb-[env(safe-area-inset-bottom)]">
                {NavPage::iter()
                    .map(|page| {
                        let path = page.path();
                        let navigate = navigate.clone();
                        view! {
                            <BottomNavButton
                                on:click=move |_| {
                                    navigate(path, Default::default());
                                }

                                attr:aria-current=move || is_active(path)
                            >
                                {page.icon()}
                                <BottomNavLabel>{page.to_string()}</BottomNavLabel>
                            </BottomNavButton>
                        }
                    })
                    .collect_view()}
            </BottomNavGrid>
        </BottomNav>
    }
}

/* ========================================================== */
/*                       ✨  ENUM  ✨                         */
/* ========================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, Default)]
enum NavPage {
    #[default]
    Home,
    Test,
    #[strum(serialize = "Gsap")]
    GsapIntro,
    Settings,
}

impl NavPage {
    fn path(self) -> &'static str {
        match self {
            NavPage::Home => "/",
            NavPage::Test => "/test",
            NavPage::GsapIntro => "/gsap-intro",
            NavPage::Settings => "/settings",
        }
    }

    fn icon(self) -> impl IntoView {
        match self {
            NavPage::Home => view! { <House class="size-5" /> }.into_any(),
            NavPage::Test => view! { <TestTube class="size-5" /> }.into_any(),
            NavPage::GsapIntro => view! { <Beaker class="size-5" /> }.into_any(),
            NavPage::Settings => view! { <Settings class="size-5" /> }.into_any(),
        }
    }
}
