use leptos::prelude::*;

#[component]
pub fn DemoLoginPage2() -> impl IntoView {
    view! {
        <div class="bg-background">
            <div class="grid lg:grid-cols-2 min-h-svh">
                <div class="flex flex-col gap-4 p-6 md:p-10">
                    <div class="flex gap-2 justify-center md:justify-start">
                        <a href="#" class="flex gap-2 items-center font-medium">
                            <div class="flex justify-center items-center rounded-md bg-primary text-primary-foreground size-6">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    class="lucide lucide-gallery-vertical-end size-4"
                                >
                                    <path d="M7 2h10"></path>
                                    <path d="M5 6h14"></path>
                                    <rect width="18" height="12" x="3" y="10" rx="2"></rect>
                                </svg>
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
                                    <div class="relative text-sm text-center after:border-border after:absolute after:inset-0 after:top-1/2 after:z-0 after:flex after:items-center after:border-t">
                                        <span class="relative z-10 px-2 bg-background text-muted-foreground">
                                            Or continue with
                                        </span>
                                    </div>
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
                                                d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"
                                                fill="currentColor"
                                            ></path>
                                        </svg>
                                        Login with GitHub
                                    </button>
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
