use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Card, div, "rounded-lg border border-input bg-card shadow p-4 w-full"}
    clx! {CardContent, div, "flex flex-col gap-4"}
    clx! {CardTitle, h3, "text-2xl font-semibold leading-none tracking-tight"}
    clx! {CardDescription, p, "text-muted-foreground"}
}

pub use components::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn TemplateDemo() -> impl IntoView {
    view! {
        <Card class="max-w-[450px]">
            <CardContent>
                <CardTitle>"Process to Follow (WIP 💪)"</CardTitle>

                <CardDescription>
                    "1. Convert the HTML TODO 👉 with Tailwind CSS (see root of the project)."
                </CardDescription>
                <CardDescription>"2. Place it in the ✔️ DONE folder."</CardDescription>
                <CardDescription>
                    "3. Create the component in the app/src/__demos__ folder and add it to page_all_demos.rs."
                </CardDescription>
                <CardDescription class="text-red-500">
                    "Please note this is susceptible to change."
                </CardDescription>
            </CardContent>
        </Card>
    }
}
