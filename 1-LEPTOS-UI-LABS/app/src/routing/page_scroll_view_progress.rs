use leptos::prelude::*;

#[component]
pub fn PageScrollViewProgress() -> impl IntoView {
    view! {
        <script src="/components/scroll_view_progress.js" />

        <div class="fixed top-0 left-0 w-full h-2 bg-gray-300">
            <div class="h-full bg-blue-500" id="readingProgress" style="width: 0;"></div>
        </div>

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
                <p class="mb-20 text-lg leading-8">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam ultricies dui ac suscipit vestibulum.
                    Pellentesque euismod turpis vel sapien bibendum, non faucibus justo placerat. In hac habitasse platea
                    dictumst. Morbi id tincidunt elit. Praesent consectetur eleifend mi, vitae gravida ante fringilla quis. Duis
                    maximus, erat metus interdum eros, in rhoncus leo augue quis quam. Sed rhoncus consequat ipsum, vel egestas
                    leo congue eu.
                </p>
                <p class="mb-20 text-lg leading-8">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam ultricies dui ac suscipit vestibulum.
                    Pellentesque euismod turpis vel sapien bibendum, non faucibus justo placerat. In hac habitasse platea
                    dictumst. Morbi id tincidunt elit. Praesent consectetur eleifend mi, vitae gravida ante fringilla quis. Duis
                    maximus, erat metus interdum eros, in rhoncus leo augue quis quam. Sed rhoncus consequat ipsum, vel egestas
                    leo congue eu.
                </p>
                <p class="mb-20 text-lg leading-8">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam ultricies dui ac suscipit vestibulum.
                    Pellentesque euismod turpis vel sapien bibendum, non faucibus justo placerat. In hac habitasse platea
                    dictumst. Morbi id tincidunt elit. Praesent consectetur eleifend mi, vitae gravida ante fringilla quis. Duis
                    maximus, erat metus interdum eros, in rhoncus leo augue quis quam. Sed rhoncus consequat ipsum, vel egestas
                    leo congue eu.
                </p>
                <p class="mb-20 text-lg leading-8">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam ultricies dui ac suscipit vestibulum.
                    Pellentesque euismod turpis vel sapien bibendum, non faucibus justo placerat. In hac habitasse platea
                    dictumst. Morbi id tincidunt elit. Praesent consectetur eleifend mi, vitae gravida ante fringilla quis. Duis
                    maximus, erat metus interdum eros, in rhoncus leo augue quis quam. Sed rhoncus consequat ipsum, vel egestas
                    leo congue eu.
                </p>
                <p class="mb-20 text-lg leading-8">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam ultricies dui ac suscipit vestibulum.
                    Pellentesque euismod turpis vel sapien bibendum, non faucibus justo placerat. In hac habitasse platea
                    dictumst. Morbi id tincidunt elit. Praesent consectetur eleifend mi, vitae gravida ante fringilla quis. Duis
                    maximus, erat metus interdum eros, in rhoncus leo augue quis quam. Sed rhoncus consequat ipsum, vel egestas
                    leo congue eu.
                </p>

            </div>
        </div>
    }
}
