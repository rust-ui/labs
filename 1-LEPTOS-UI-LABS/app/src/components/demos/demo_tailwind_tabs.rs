use leptos::prelude::*;

use crate::components::ui::tailwind_tabs::{
    SubTabsList, TabSubTriggers, Tabs, TabsList, TabsTrigger,
};

#[allow(non_upper_case_globals)]
#[component]
pub fn DemoTailwindTabs() -> impl IntoView {
    const TAB_1: &str = "radio-tab-1";
    const TAB_2: &str = "radio-tab-2";
    const TAB_3: &str = "radio-tab-3";

    const NAME_TABS: &str = "sub-tab-toggle";

    const PANEL_1: &str = "panel-toggle-1";
    const PANEL_2: &str = "panel-toggle-2";
    const PANEL_3: &str = "panel-toggle-3";

    const PANEL__1_1: &str = "panel-1.1";
    const PANEL__1_2: &str = "panel-1.2";
    const PANEL__1_3: &str = "panel-1.3";

    const PANEL__2_1: &str = "panel-2.1";
    const PANEL__2_2: &str = "panel-2.2";
    const PANEL__2_3: &str = "panel-2.3";

    const PANEL__3_1: &str = "panel-3.1";
    const PANEL__3_2: &str = "panel-3.2";
    const PANEL__3_3: &str = "panel-3.3";

    const PANEL_CONTENT_1: &str = "panels-contents-1";
    const PANEL_CONTENT_2: &str = "panels-contents-2";
    const PANEL_CONTENT_3: &str = "panels-contents-3";

    const CLASS_TABS_CONTENT: &str = "grid mt-2 [grid-template-areas:'contents'] *:[grid-area:contents] *:transition-opacity *:duration-300 *:rounded-md *:grid *:[grid-template-areas:'panels'] *:opacity-0 [&>div>div]:[grid-area:panels] [&>div>div]:transition-opacity [&>div>div]:duration-300 [&>div>div]:opacity-0 [&>div>div]:p-8 [&>div>div]:grid [&>div>div]:place-content-center peer-checked/panel-1.1:[&_#panel-11]:opacity-100 peer-checked/panel-1.2:[&_#panel-12]:opacity-100 peer-checked/panel-1.3:[&_#panel-13]:opacity-100 peer-checked/panel-2.1:[&_#panel-21]:opacity-100 peer-checked/panel-2.2:[&_#panel-22]:opacity-100 peer-checked/panel-2.3:[&_#panel-23]:opacity-100 peer-checked/panel-3.1:[&_#panel-31]:opacity-100 peer-checked/panel-3.2:[&_#panel-32]:opacity-100 peer-checked/panel-3.3:[&_#panel-33]:opacity-100";

    view! {
        <Tabs>
            <TabsTrigger id=TAB_1 name=NAME_TABS class="peer/tab-1" checked=true />
            <TabsTrigger id=TAB_2 name=NAME_TABS class="peer/tab-2" />
            <TabsTrigger id=TAB_3 name=NAME_TABS class="peer/tab-3" />

            <TabsList>
                <label for=TAB_1>Tab 1</label>
                <label for=TAB_2>Tab 2</label>
                <label for=TAB_3>Tab 3</label>
            </TabsList>

            <SubTabsList>
                <TabsTrigger id=PANEL__1_1 name=PANEL_1 class="peer/panel-1.1" checked=true />
                <TabsTrigger id=PANEL__1_2 name=PANEL_1 class="peer/panel-1.2" />
                <TabsTrigger id=PANEL__1_3 name=PANEL_1 class="peer/panel-1.3" />

                <TabsTrigger id=PANEL__2_1 name=PANEL_2 class="peer/panel-2.1" checked=true />
                <TabsTrigger id=PANEL__2_2 name=PANEL_2 class="peer/panel-2.2" />
                <TabsTrigger id=PANEL__2_3 name=PANEL_2 class="peer/panel-2.3" />

                <TabsTrigger id=PANEL__3_1 name=PANEL_3 class="peer/panel-3.1" checked=true />
                <TabsTrigger id=PANEL__3_2 name=PANEL_3 class="peer/panel-3.2" />
                <TabsTrigger id=PANEL__3_3 name=PANEL_3 class="peer/panel-3.3" />

                <TabSubTriggers>
                    <div id="sub-tab-1">
                        <label for=PANEL__1_1>SubTab 1.1</label>
                        <label for=PANEL__1_2>SubTab 1.2</label>
                        <label for=PANEL__1_3>SubTab 1.3</label>
                    </div>

                    <div id="sub-tab-2">
                        <label for=PANEL__2_1>SubTab 2.1</label>
                        <label for=PANEL__2_2>SubTab 2.2</label>
                        <label for=PANEL__2_3>SubTab 2.3</label>
                    </div>

                    <div id="sub-tab-3">
                        <label for=PANEL__3_1>SubTab 3.1</label>
                        <label for=PANEL__3_2>SubTab 3.2</label>
                        <label for=PANEL__3_3>SubTab 3.3</label>
                    </div>
                </TabSubTriggers>

                <div class=CLASS_TABS_CONTENT>
                    <div id=PANEL_CONTENT_1 class="bg-blue-800">
                        <div id="panel-11">panel 1.1</div>
                        <div id="panel-12">panel 1.2</div>
                        <div id="panel-13">panel 1.3</div>
                    </div>

                    <div id=PANEL_CONTENT_2 class="bg-green-800">
                        <div id="panel-21">panel 2.1</div>
                        <div id="panel-22">panel 2.2</div>
                        <div id="panel-23">panel 2.3</div>
                    </div>

                    <div id=PANEL_CONTENT_3 class="bg-rose-800">
                        <div id="panel-31">panel 3.1</div>
                        <div id="panel-32">panel 3.2</div>
                        <div id="panel-33">panel 3.3</div>
                    </div>
                </div>
            </SubTabsList>
        </Tabs>
    }
}
