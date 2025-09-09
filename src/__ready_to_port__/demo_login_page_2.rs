use icons::GalleryVerticalEnd;
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;

#[component]
pub fn DemoLoginPage2() -> impl IntoView {
    view! {
        <div class="bg-background">
            <div class="grid lg:grid-cols-2 min-h-svh">
                <div class="flex flex-col gap-4 p-6 md:p-10">
                    <div class="flex gap-2 justify-center md:justify-start">
                        <a href="#" class="flex gap-2 items-center font-medium">
                            <div class="flex justify-center items-center rounded-md bg-primary text-primary-foreground size-6">
                                <GalleryVerticalEnd class="size-4" />
                            </div>
                            Acme Inc.
                        </a>
                    </div>
                    <div class="flex flex-1 justify-center items-center">
                        <div class="w-full max-w-xs">
                            <form class="flex flex-col gap-6">
                                <div class="flex flex-col gap-2 items-center text-center">
                                    <h1 class="text-2xl font-bold">Login to your account</h1>
                                    <p class="text-sm text-muted-foreground text-balance">
                                        Enter your email below to login to your account
                                    </p>
                                </div>
                                <div class="grid gap-6">
                                    <div class="grid gap-3">
                                        <Label html_for="email">Email</Label>
                                        <Input
                                            r#type="email"
                                            id="email"
                                            placeholder="m@example.com"
                                            required=true
                                        />
                                    </div>
                                    <div class="grid gap-3">
                                        <div class="flex items-center">
                                            <Label html_for="password">Password</Label>
                                            <a
                                                href="#"
                                                class="ml-auto text-sm hover:underline underline-offset-4"
                                            >
                                                Forgot your password?
                                            </a>
                                        </div>
                                        <Input r#type="password" id="password" required=true />
                                    </div>

                                    <Button class="w-full" r#type="submit">
                                        Login
                                    </Button>
                                    <div class="relative text-sm text-center after:border-border after:absolute after:inset-0 after:top-1/2 after:z-0 after:flex after:items-center after:border-t">
                                        <span class="relative z-10 px-2 bg-background text-muted-foreground">
                                            Or continue with
                                        </span>
                                    </div>
                                    <Button variant=ButtonVariant::Outline class="w-full">
                                        <GithubSvg />
                                        <span>Login with GitHub</span>
                                    </Button>
                                </div>
                                <div class="text-sm text-center">
                                    "Don't have an account?"
                                    <a href="#" class="underline underline-offset-4">
                                        Sign up
                                    </a>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
                <div class="hidden relative lg:block bg-muted">
                    <img
                        src="/placeholder.svg"
                        alt="Image"
                        class="object-cover absolute inset-0 w-full h-full dark:brightness-[0.2] dark:grayscale"
                    />
                </div>
            </div>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn GithubSvg() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
                d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"
                fill="currentColor"
            ></path>
        </svg>
    }
}
