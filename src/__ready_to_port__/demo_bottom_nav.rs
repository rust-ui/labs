use icons::{CircleUser, House, SlidersHorizontal, Wallet};
use leptos::prelude::*;

#[component]
pub fn DemoBottomNav() -> impl IntoView {
    view! {
        <div class="flex flex-col my-10 rounded-t-2xl border h-[300px] w-[400px]">
            <div class="flex-1 bg-gray-200 rounded-t-2xl"></div>

            <nav class="mx-auto w-full max-w-lg h-16 bg-white border-t border-gray-200 dark:bg-gray-700 dark:border-gray-600">
                <div class="grid grid-cols-4 h-full font-medium">
                    <button
                        type="button"
                        class="inline-flex flex-col justify-center items-center px-5 hover:bg-gray-50 group dark:hover:bg-gray-800"
                    >
                        <House class="mb-2 w-5 h-5 text-gray-500 dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-500" />
                        <span class="text-sm text-gray-500 dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-500">
                            "Home"
                        </span>
                    </button>

                    <button
                        type="button"
                        class="inline-flex flex-col justify-center items-center px-5 hover:bg-gray-50 group dark:hover:bg-gray-800"
                    >
                        <Wallet class="mb-2 w-5 h-5 text-gray-500 dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-500" />
                        <span class="text-sm text-gray-500 dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-500">
                            "Wallet"
                        </span>
                    </button>

                    <button
                        type="button"
                        class="inline-flex flex-col justify-center items-center px-5 hover:bg-gray-50 group dark:hover:bg-gray-800"
                    >
                        <SlidersHorizontal class="mb-2 w-5 h-5 text-gray-500 dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-500" />
                        <span class="text-sm text-gray-500 dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-500">
                            "Settings"
                        </span>
                    </button>

                    <button
                        type="button"
                        class="inline-flex flex-col justify-center items-center px-5 hover:bg-gray-50 group dark:hover:bg-gray-800"
                    >
                        <CircleUser class="mb-2 w-5 h-5 text-gray-500 dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-500" />
                        <span class="text-sm text-gray-500 dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-500">
                            "Profile"
                        </span>
                    </button>
                </div>
            </nav>
        </div>
    }
}
