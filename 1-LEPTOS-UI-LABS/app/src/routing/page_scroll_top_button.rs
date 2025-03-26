use leptos::prelude::*;

#[component]
pub fn PageScrollTopButton() -> impl IntoView {
    view! {
        <script src="/components/scroll_top_button.js" />

        <div>
            <div class="max-w-4xl p-8 mx-auto">
                <h1 class="mb-8 text-4xl font-bold">Scroll to view effect</h1>
                <div class="flex items-center mb-6">
                    <div class="w-8 h-8 mr-4 bg-gray-400 rounded-full"></div>
                    <div>
                        <div class="w-24 h-2 mb-1 bg-gray-400"></div>
                        <div class="w-16 h-2 bg-gray-400"></div>
                    </div>
                </div>
                <p class="mb-24 text-lg leading-8">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam ultricies dui ac suscipit vestibulum.
                    Pellentesque euismod turpis vel sapien bibendum, non faucibus justo placerat. In hac habitasse platea
                    dictumst. Morbi id tincidunt elit. Praesent consectetur eleifend mi, vitae gravida ante fringilla quis. Duis
                    maximus, erat metus interdum eros, in rhoncus leo augue quis quam. Sed rhoncus consequat ipsum, vel egestas
                    leo congue eu.
                </p>
                <p class="mb-24 text-lg leading-8">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam ultricies dui ac suscipit vestibulum.
                    Pellentesque euismod turpis vel sapien bibendum, non faucibus justo placerat. In hac habitasse platea
                    dictumst. Morbi id tincidunt elit. Praesent consectetur eleifend mi, vitae gravida ante fringilla quis. Duis
                    maximus, erat metus interdum eros, in rhoncus leo augue quis quam. Sed rhoncus consequat ipsum, vel egestas
                    leo congue eu.
                </p>
                <p class="mb-24 text-lg leading-8">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam ultricies dui ac suscipit vestibulum.
                    Pellentesque euismod turpis vel sapien bibendum, non faucibus justo placerat. In hac habitasse platea
                    dictumst. Morbi id tincidunt elit. Praesent consectetur eleifend mi, vitae gravida ante fringilla quis. Duis
                    maximus, erat metus interdum eros, in rhoncus leo augue quis quam. Sed rhoncus consequat ipsum, vel egestas
                    leo congue eu.
                </p>
                <p class="mb-24 text-lg leading-8">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam ultricies dui ac suscipit vestibulum.
                    Pellentesque euismod turpis vel sapien bibendum, non faucibus justo placerat. In hac habitasse platea
                    dictumst. Morbi id tincidunt elit. Praesent consectetur eleifend mi, vitae gravida ante fringilla quis. Duis
                    maximus, erat metus interdum eros, in rhoncus leo augue quis quam. Sed rhoncus consequat ipsum, vel egestas
                    leo congue eu.
                </p>
                <p class="mb-24 text-lg leading-8">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam ultricies dui ac suscipit vestibulum.
                    Pellentesque euismod turpis vel sapien bibendum, non faucibus justo placerat. In hac habitasse platea
                    dictumst. Morbi id tincidunt elit. Praesent consectetur eleifend mi, vitae gravida ante fringilla quis. Duis
                    maximus, erat metus interdum eros, in rhoncus leo augue quis quam. Sed rhoncus consequat ipsum, vel egestas
                    leo congue eu.
                </p>

            </div>
        </div>

        <button
            id="to-top-button"
            onclick="goToTop()"
            title="Go To Top"
            class="fixed z-50 hidden p-4 text-lg font-semibold text-white transition-colors duration-300 bg-purple-600 border-0 rounded-full shadow-md bottom-10 right-10 w-14 h-14 hover:bg-purple-700"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="currentColor"
                class="w-6 h-6"
            >
                <path d="M12 4l8 8h-6v8h-4v-8H4l8-8z" />
            </svg>
            <span class="sr-only">Go to top</span>
        </button>
    }
}
