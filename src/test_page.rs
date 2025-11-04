use icons::{HeartAnimate, PlusAnimate};
use leptos::prelude::*;
use leptos_ui::clx;

#[component]
pub fn TestPage() -> impl IntoView {
    clx! {IconsWrapper, div, "group", "flex justify-center items-center p-2 rounded-md transition-colors duration-200 cursor-pointer select-none  hover:bg-accent"}

    view! {
        <div>
            <h1>Test Page</h1>

            <div class="flex gap-8 justify-center items-center m-0 min-h-screen font-sans">
                <IconsWrapper>
                    <HeartAnimate />
                </IconsWrapper>

                <PlusAnimate />

                <EmailNewComponents />
            </div>
        </div>
    }
}

#[component]
pub fn EmailNewComponents() -> impl IntoView {
    view! {
        <div class="overflow-hidden relative py-14 px-4 rounded-xl sm:px-8 bg-zinc-900 dark">
            <svg
                class="absolute top-0 left-0 -translate-x-1/2"
                xmlns="http://www.w3.org/2000/svg"
                width="267"
                height="268"
                fill="none"
                aria-hidden="true"
            >
                <g filter="url(#_r_55_a)" style="mix-blend-mode: plus-lighter;">
                    <path
                        fill="#fff"
                        fill-opacity=".48"
                        d="M189 76.284 242.642 24 189 83.753v19.691l-8.148-6.11L24 244 176.099 89.864v-13.58H189Z"
                    ></path>
                </g>
                <defs>
                    <filter
                        id="_r_55_a"
                        width="266.642"
                        height="268"
                        x="0"
                        y="0"
                        color-interpolation-filters="sRGB"
                        filterUnits="userSpaceOnUse"
                    >
                        <feFlood flood-opacity="0" result="BackgroundImageFix"></feFlood>
                        <feBlend
                            in="SourceGraphic"
                            in2="BackgroundImageFix"
                            result="shape"
                        ></feBlend>
                        <feGaussianBlur
                            result="effect1_foregroundBlur_809_24"
                            stdDeviation="12"
                        ></feGaussianBlur>
                    </filter>
                </defs>
            </svg>
            <svg
                class="absolute right-0 bottom-0 translate-x-1/4"
                xmlns="http://www.w3.org/2000/svg"
                width="267"
                height="268"
                fill="none"
                aria-hidden="true"
            >
                <g filter="url(#_r_56_a)" style="mix-blend-mode: plus-lighter;">
                    <path
                        fill="#fff"
                        fill-opacity=".48"
                        d="M189 76.284 242.642 24 189 83.753v19.691l-8.148-6.11L24 244 176.099 89.864v-13.58H189Z"
                    ></path>
                </g>
                <defs>
                    <filter
                        id="_r_56_a"
                        width="266.642"
                        height="268"
                        x="0"
                        y="0"
                        color-interpolation-filters="sRGB"
                        filterUnits="userSpaceOnUse"
                    >
                        <feFlood flood-opacity="0" result="BackgroundImageFix"></feFlood>
                        <feBlend
                            in="SourceGraphic"
                            in2="BackgroundImageFix"
                            result="shape"
                        ></feBlend>
                        <feGaussianBlur
                            result="effect1_foregroundBlur_809_24"
                            stdDeviation="12"
                        ></feGaussianBlur>
                    </filter>
                </defs>
            </svg>
            <div class="flex flex-col gap-6 justify-between items-center lg:flex-row">
                <h2 class="font-heading text-2xl/[1.1] text-foreground md:text-3xl/[1.1]">
                    Get notified when new stuff drops.
                </h2>
                <form class="space-y-4">
                    <div class="space-y-2">
                        <div class="inline-flex gap-2">
                            <div class="relative">
                                <div class="flex absolute inset-y-0 justify-center items-center pointer-events-none start-0 ps-3 text-muted-foreground peer-disabled:opacity-50">
                                    <svg
                                        viewBox="0 0 24 24"
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="16"
                                        height="16"
                                        fill="currentColor"
                                        aria-hidden="true"
                                        class="remixicon -scale-x-100"
                                    >
                                        <path d="M4.96488 5.09625L8.5107 17.5066L11.5514 11.4253L17.188 9.17062L4.96488 5.09625ZM2.89945 2.29958L21.7052 8.56818C21.9672 8.6555 22.1088 8.93866 22.0215 9.20063C21.975 9.34016 21.8694 9.45214 21.7328 9.50676L13.0002 12.9998L8.57501 21.8501C8.45151 22.0971 8.15118 22.1972 7.90419 22.0737C7.77883 22.011 7.68553 21.8986 7.64703 21.7639L2.26058 2.91129C2.18472 2.64577 2.33846 2.36903 2.60398 2.29316C2.70087 2.26548 2.80386 2.26772 2.89945 2.29958Z"></path>
                                    </svg>
                                </div>
                                <input
                                    data-slot="input"
                                    class="flex flex-1 py-1 px-3 w-full min-w-0 h-10 text-sm rounded-full border outline-none disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none shadow-xs transition-[color,box-shadow] file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground aria-invalid:border-destructive aria-invalid:ring-destructive/20 border-zinc-600/65 bg-zinc-700/30 ps-9 text-zinc-100 placeholder:text-zinc-500 [&amp;:-webkit-autofill]:bg-zinc-700/30 [&amp;:-webkit-autofill]:[-webkit-text-fill-color:#fff] [&amp;:-webkit-autofill]:[transition:background-color_5000000s_ease-in-out_0s] md:min-w-64 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50"
                                    id="_r_57_"
                                    placeholder="Enter your email..."
                                    aria-label="Subscribe to the newsletter"
                                    required=""
                                    type="email"
                                    value=""
                                />
                            </div>
                            <button
                                data-slot="button"
                                class="inline-flex relative gap-2 justify-center items-center py-2 px-4 h-10 text-sm font-medium whitespace-nowrap rounded-full shadow-sm outline-none disabled:opacity-50 disabled:pointer-events-none transition-[color,box-shadow] [&amp;_svg]:pointer-events-none [&amp;_svg]:shrink-0 [&amp;_svg:not([class*='size-'])]:size-4 bg-primary text-primary-foreground group hover:bg-primary/90 focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50"
                                type="submit"
                                data-loading="false"
                            >
                                <span class="group-data-[loading=true]:text-transparent">
                                    Subscribe
                                </span>
                            </button>
                        </div>
                    </div>
                </form>
            </div>
        </div>
    }
}
