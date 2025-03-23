use leptos::prelude::*;

use crate::components::ui::tabs::{Tabs, TabsContent, TabsTrigger};

#[component]
pub fn DemoTabs() -> impl IntoView {
    view! {
        <style>
            {".tabs__item-input:checked + .tabs__item-label {
            border-color: white;
            }
            .tabs__item-input:checked + .tabs__item-label + .tabs__item-content {
            display: block;
            }
            "}
        </style>

        <Tabs>
            <TabsTrigger checked=true>Tab One</TabsTrigger>
            <TabsContent>
                <h2>Tab One Content</h2>
                <p>{RANDOM_TEXT}</p>
                <p>{RANDOM_TEXT}</p>
                <p>{RANDOM_TEXT}</p>
            </TabsContent>

            <TabsTrigger>Tab Two</TabsTrigger>
            <TabsContent>
                <h2>Tab Two Content</h2>
                <p>{RANDOM_TEXT}</p>
                <p>{RANDOM_TEXT}</p>
            </TabsContent>

            <TabsTrigger>Tab Three</TabsTrigger>
            <TabsContent>
                <h2>Tab Three Content</h2>
                <p>{RANDOM_TEXT}</p>
            </TabsContent>
        </Tabs>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const RANDOM_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.";
