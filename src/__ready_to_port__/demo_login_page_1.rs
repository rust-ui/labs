use leptos::prelude::*;

#[component]
pub fn DemoLoginPage1() -> impl IntoView {
    view! {
        <div class="bg-background">
            <div class="flex justify-center items-center p-6 w-full md:p-10 min-h-svh">
                <div class="w-full max-w-sm">
                    <div class="flex flex-col gap-6">
                        <div
                            data-slot="card"
                            class="flex flex-col gap-6 py-6 rounded-xl border shadow-sm bg-card text-card-foreground"
                        >
                            <div
                                data-slot="card-header"
                                class="grid auto-rows-min gap-1.5 items-start px-6 @container/card-header grid-rows-[auto_auto] has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6"
                            >
                                <div data-slot="card-title" class="font-semibold leading-none">
                                    Login to your account
                                </div>
                                <div
                                    data-slot="card-description"
                                    class="text-sm text-muted-foreground"
                                >
                                    Enter your email below to login to your account
                                </div>
                            </div>
                            <div data-slot="card-content" class="px-6">
                                <form>
                                    <div class="flex flex-col gap-6">
                                        <div class="grid gap-3">
                                            <label
                                                data-slot="label"
                                                class="flex gap-2 items-center text-sm font-medium leading-none select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50"
                                                for="email"
                                            >
                                                Email
                                            </label>
                                            <input
                                                type="email"
                                                data-slot="input"
                                                class="flex py-1 px-3 w-full min-w-0 h-9 text-base bg-transparent rounded-md border outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground border-input shadow-xs transition-[color,box-shadow] file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium aria-invalid:ring-destructive/20 aria-invalid:border-destructive dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                                id="email"
                                                placeholder="m@example.com"
                                                required=""
                                            />
                                        </div>
                                        <div class="grid gap-3">
                                            <div class="flex items-center">
                                                <label
                                                    data-slot="label"
                                                    class="flex gap-2 items-center text-sm font-medium leading-none select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50"
                                                    for="password"
                                                >
                                                    Password
                                                </label>
                                                <a
                                                    href="#"
                                                    class="inline-block ml-auto text-sm hover:underline underline-offset-4"
                                                >
                                                    Forgot your password?
                                                </a>
                                            </div>
                                            <input
                                                type="password"
                                                data-slot="input"
                                                class="flex py-1 px-3 w-full min-w-0 h-9 text-base bg-transparent rounded-md border outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground border-input shadow-xs transition-[color,box-shadow] file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium aria-invalid:ring-destructive/20 aria-invalid:border-destructive dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                                id="password"
                                                required=""
                                            />
                                        </div>
                                        <div class="flex flex-col gap-3">
                                            <button
                                                data-slot="button"
                                                class="inline-flex gap-2 justify-center items-center py-2 px-4 w-full h-9 text-sm font-medium whitespace-nowrap rounded-md transition-all outline-none disabled:opacity-50 disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*='size-'])]:size-4 shrink-0 [&amp;_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive bg-primary text-primary-foreground shadow-xs has-[&gt;svg]:px-3 dark:aria-invalid:ring-destructive/40 hover:bg-primary/90 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                                type="submit"
                                            >
                                                Login
                                            </button>
                                            <button
                                                data-slot="button"
                                                class="inline-flex gap-2 justify-center items-center py-2 px-4 w-full h-9 text-sm font-medium whitespace-nowrap rounded-md border transition-all outline-none disabled:opacity-50 disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*='size-'])]:size-4 shrink-0 [&amp;_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive bg-background shadow-xs has-[&gt;svg]:px-3 dark:aria-invalid:ring-destructive/40 dark:bg-input/30 dark:border-input dark:hover:bg-input/50 hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                            >
                                                Login with Google
                                            </button>
                                        </div>
                                    </div>
                                    <div class="mt-4 text-sm text-center">
                                        "Don't have an account?"
                                        <a href="#" class="underline underline-offset-4">
                                            Sign up
                                        </a>
                                    </div>
                                </form>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
