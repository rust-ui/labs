use icons::GalleryVerticalEnd;
use leptos::prelude::*;

#[component]
pub fn DemoLoginPage3() -> impl IntoView {
    view! {
        <div class="bg-background">
            <div class="flex flex-col gap-6 justify-center items-center p-6 md:p-10 bg-muted min-h-svh">
                <div class="flex flex-col gap-6 w-full max-w-sm">
                    <a href="#" class="flex gap-2 items-center self-center font-medium">
                        <div class="flex justify-center items-center rounded-md bg-primary text-primary-foreground size-6">
                            <GalleryVerticalEnd class="size-4" />
                        </div>
                        Acme Inc.
                    </a>
                    <div class="flex flex-col gap-6">
                        <div
                            data-slot="card"
                            class="flex flex-col gap-6 py-6 rounded-xl border shadow-sm bg-card text-card-foreground"
                        >
                            <div
                                data-slot="card-header"
                                class="grid auto-rows-min gap-1.5 items-start px-6 text-center @container/card-header grid-rows-[auto_auto] has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6"
                            >
                                <div data-slot="card-title" class="text-xl font-semibold">
                                    Welcome back
                                </div>
                                <div
                                    data-slot="card-description"
                                    class="text-sm text-muted-foreground"
                                >
                                    Login with your Apple or Google account
                                </div>
                            </div>
                            <div data-slot="card-content" class="px-6">
                                <form>
                                    <div class="grid gap-6">
                                        <div class="flex flex-col gap-4">
                                            <button
                                                data-slot="button"
                                                class="inline-flex gap-2 justify-center items-center py-2 px-4 w-full h-9 text-sm font-medium whitespace-nowrap rounded-md border transition-all outline-none disabled:opacity-50 disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*='size-'])]:size-4 shrink-0 [&amp;_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive bg-background shadow-xs has-[&gt;svg]:px-3 dark:aria-invalid:ring-destructive/40 dark:bg-input/30 dark:border-input dark:hover:bg-input/50 hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                            >
                                                <svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    viewBox="0 0 24 24"
                                                    class="size-4"
                                                >
                                                    <path
                                                        d="M12.152 6.896c-.948 0-2.415-1.078-3.96-1.04-2.04.027-3.91 1.183-4.961 3.014-2.117 3.675-.546 9.103 1.519 12.09 1.013 1.454 2.208 3.09 3.792 3.039 1.52-.065 2.09-.987 3.935-.987 1.831 0 2.35.987 3.96.948 1.637-.026 2.676-1.48 3.676-2.948 1.156-1.688 1.636-3.325 1.662-3.415-.039-.013-3.182-1.221-3.22-4.857-.026-3.04 2.48-4.494 2.597-4.559-1.429-2.09-3.623-2.324-4.39-2.376-2-.156-3.675 1.09-4.61 1.09zM15.53 3.83c.843-1.012 1.4-2.427 1.245-3.83-1.207.052-2.662.805-3.532 1.818-.78.896-1.454 2.338-1.273 3.714 1.338.104 2.715-.688 3.559-1.701"
                                                        fill="currentColor"
                                                    ></path>
                                                </svg>
                                                Login with Apple
                                            </button>
                                            <button
                                                data-slot="button"
                                                class="inline-flex gap-2 justify-center items-center py-2 px-4 w-full h-9 text-sm font-medium whitespace-nowrap rounded-md border transition-all outline-none disabled:opacity-50 disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*='size-'])]:size-4 shrink-0 [&amp;_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive bg-background shadow-xs has-[&gt;svg]:px-3 dark:aria-invalid:ring-destructive/40 dark:bg-input/30 dark:border-input dark:hover:bg-input/50 hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                            >
                                                <svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    viewBox="0 0 24 24"
                                                    class="size-4"
                                                >
                                                    <path
                                                        d="M12.48 10.92v3.28h7.84c-.24 1.84-.853 3.187-1.787 4.133-1.147 1.147-2.933 2.4-6.053 2.4-4.827 0-8.6-3.893-8.6-8.72s3.773-8.72 8.6-8.72c2.6 0 4.507 1.027 5.907 2.347l2.307-2.307C18.747 1.44 16.133 0 12.48 0 5.867 0 .307 5.387.307 12s5.56 12 12.173 12c3.573 0 6.267-1.173 8.373-3.36 2.16-2.16 2.84-5.213 2.84-7.667 0-.76-.053-1.467-.173-2.053H12.48z"
                                                        fill="currentColor"
                                                    ></path>
                                                </svg>
                                                Login with Google
                                            </button>
                                        </div>
                                        <div class="relative text-sm text-center after:border-border after:absolute after:inset-0 after:top-1/2 after:z-0 after:flex after:items-center after:border-t">
                                            <span class="relative z-10 px-2 bg-card text-muted-foreground">
                                                Or continue with
                                            </span>
                                        </div>
                                        <div class="grid gap-6">
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
                                                        class="ml-auto text-sm hover:underline underline-offset-4"
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
                                            <button
                                                data-slot="button"
                                                class="inline-flex gap-2 justify-center items-center py-2 px-4 w-full h-9 text-sm font-medium whitespace-nowrap rounded-md transition-all outline-none disabled:opacity-50 disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg:not([class*='size-'])]:size-4 shrink-0 [&amp;_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive bg-primary text-primary-foreground shadow-xs has-[&gt;svg]:px-3 dark:aria-invalid:ring-destructive/40 hover:bg-primary/90 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                                type="submit"
                                            >
                                                Login
                                            </button>
                                        </div>
                                        <div class="text-sm text-center">
                                            "Don't have an account?"
                                            <a href="#" class="underline underline-offset-4">
                                                Sign up
                                            </a>
                                        </div>
                                    </div>
                                </form>
                            </div>
                        </div>
                        <div class="text-xs text-center text-muted-foreground *:[a]:hover:text-primary text-balance *:[a]:underline *:[a]:underline-offset-4">
                            By clicking continue, you agree to our <a href="#">Terms of Service</a>
                            and <a href="#">Privacy Policy</a>.
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
