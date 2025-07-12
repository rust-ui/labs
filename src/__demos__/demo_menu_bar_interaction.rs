use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoMenuBarInteraction() -> impl IntoView {
    view! {
        <Stylesheet href="/components/menu_bar_interaction.css" />

        <div class="mainDiv">
            <div id="js-chatbar" class="chat-bar">
                <div id="js-toggle" class="chat-bar__toggle" onclick="toggle()">
                    <i class="material-icons">add</i>
                </div>
                <div class="chat-bar__message">
                    <input class="chat-bar__input" type="text" placeholder="Message..." />
                </div>
                <div class="chat-bar__buttons">
                    <div class="button b-1">
                        <i class="material-icons">camera_alt</i>
                    </div>
                    <div class="button b-2">
                        <i class="material-icons">photo</i>
                    </div>
                    <div class="button b-3">
                        <i class="material-icons">gif</i>
                    </div>
                    <div class="button b-4">
                        <i class="material-icons">more_horiz</i>
                    </div>
                </div>
            </div>
        </div>

        <script src="/components/menu_bar_interaction.js"></script>
    }
}
