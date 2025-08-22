use leptos::prelude::*;

use crate::components::ui::card::Card;

#[component]
pub fn Card__ComponentsPortToRustUI() -> impl IntoView {
    view! {
        <Card class="flex flex-col gap-4 px-4 pb-6 mx-auto bg-orange-50 w-fit">
            <h1 class="text-2xl font-bold text-center">"🤖 Components Port to Rust UI"</h1>

            <div class="flex flex-col gap-4">
                <p>"🚧 WIP. Full automation using CLAUDE. Coming soon! 🙂"</p>
                <p>"You can already check .claude/commands/*."</p>
            </div>

        </Card>
    }
}
