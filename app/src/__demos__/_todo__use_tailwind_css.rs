use leptos::prelude::*;

use crate::shared::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn Todo__UseTailwindCss() -> impl IntoView {
    view! {
        <Card class="flex flex-col gap-4 pb-6 mx-auto bg-orange-50 w-fit">
            <h1 class="text-2xl font-bold text-center">"👉 TODO"</h1>

            <div class="flex items-center gap-6">
                <div class="flex flex-col gap-2">
                    <p>"1. Use Tailwind CSS instead of plain CSS (see Stylesheet `src`)."</p>
                    <p>"2. Use `clx!` macro to create components."</p>
                    <p>"3. When it's done, replace '👉' by '✔️' in `all_demos.rs`."</p>
                    <p>"4. Create the [`PR`] on Github. Thanks! 😄"</p>
                </div>

                <Card class="max-w-[400px] bg-green-50 p-2">
                    <CardContent>
                        <CardHeader>
                            <CardTitle>"TIPS 💡"</CardTitle>
                            <CardDescription>
                                "1. Use `.cursor/rules` to go much faster. "
                            </CardDescription>
                            <CardDescription>
                                "2. If you don't find the equivalent in Tailwind, leave as it is."
                            </CardDescription>
                            <CardDescription>
                                "3. Use `clx!` ONLY if it's relevant to the component itself."
                            </CardDescription>
                            <CardDescription>"└─> Thanks! 🙏"</CardDescription>
                        </CardHeader>
                    </CardContent>
                </Card>
            </div>
        </Card>
    }
}
