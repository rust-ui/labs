use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::*;
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;

#[component]
pub fn DemoLoginPage1() -> impl IntoView {
    view! {
        <div class="bg-background">
            <div class="flex justify-center items-center p-6 w-full md:p-10 min-h-svh">
                <div class="w-full max-w-sm">
                    <div class="flex flex-col gap-6">
                        <Card class="flex flex-col gap-6 py-6 rounded-xl border shadow-sm bg-card text-card-foreground">
                            <CardHeader class="grid auto-rows-min gap-1.5 items-start px-6 @container/card-header grid-rows-[auto_auto] has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6">
                                <CardTitle class="font-semibold leading-none">
                                    Login to your account
                                </CardTitle>
                                <CardDescription class="text-sm text-muted-foreground">
                                    Enter your email below to login to your account
                                </CardDescription>
                            </CardHeader>
                            <CardContent class="px-6">
                                <form>
                                    <div class="flex flex-col gap-6">
                                        <div class="grid gap-3">
                                            <Label
                                                html_for="email"
                                                class="flex gap-2 items-center select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50"
                                            >
                                                Email
                                            </Label>
                                            <Input
                                                r#type="email"
                                                id="email"
                                                placeholder="m@example.com"
                                                required=true
                                                class="py-1 h-9 text-base bg-transparent md:text-sm selection:bg-primary selection:text-primary-foreground shadow-xs transition-[color,box-shadow] aria-invalid:ring-destructive/20 aria-invalid:border-destructive dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                            />
                                        </div>
                                        <div class="grid gap-3">
                                            <div class="flex items-center">
                                                <Label
                                                    html_for="password"
                                                    class="flex gap-2 items-center select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50"
                                                >
                                                    Password
                                                </Label>
                                                <a
                                                    href="#"
                                                    class="inline-block ml-auto text-sm hover:underline underline-offset-4"
                                                >
                                                    Forgot your password?
                                                </a>
                                            </div>
                                            <Input
                                                r#type="password"
                                                id="password"
                                                required=true
                                                class="py-1 h-9 text-base bg-transparent md:text-sm selection:bg-primary selection:text-primary-foreground shadow-xs transition-[color,box-shadow] aria-invalid:ring-destructive/20 aria-invalid:border-destructive dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                            />
                                        </div>
                                        <div class="flex flex-col gap-3">
                                            <Button
                                                variant=ButtonVariant::Default
                                                r#type="submit"
                                                class="w-full"
                                            >
                                                Login
                                            </Button>
                                            <Button
                                                variant=ButtonVariant::Outline
                                                class="w-full bg-background dark:bg-input/30 dark:border-input dark:hover:bg-input/50"
                                            >
                                                Login with Google
                                            </Button>
                                        </div>
                                    </div>
                                    <div class="mt-4 text-sm text-center">
                                        "Don't have an account?"
                                        <a href="#" class="underline underline-offset-4">
                                            Sign up
                                        </a>
                                    </div>
                                </form>
                            </CardContent>
                        </Card>
                    </div>
                </div>
            </div>
        </div>
    }
}
