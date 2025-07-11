use leptos::*;
use leptos::prelude::ElementChild;
use leptos::prelude::ClassAttribute;

#[component]
pub fn HeroTwo() -> impl IntoView {
    view! {
        <section class="text-gray-700 body-font bg-gradient-to-r from-gray-50 to-gray-100">
             <div class="container mx-auto flex flex-col-reverse md:flex-col px-5 py-24 items-center">
             <div class="flex flex-col items-center text-center mb-12 md:mb-8">
              <h1 class="title-font sm:text-4xl text-3xl mb-4 font-extrabold text-gray-900">
                "Build blazing-fast UI with"
                <br class="hidden lg:inline-block"/>
                "Rustify — The Rust UI Library"
              </h1>
              <p class="mb-8 leading-relaxed text-lg text-gray-600">
                "Empower your Rust applications with sleek, type-safe, and reactive components. Experience seamless integration, zero-cost abstractions, and unmatched performance."
              </p>
              <div class="flex justify-center">
                <button class="inline-flex text-white bg-indigo-600 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-700 rounded text-lg font-semibold">
                  "Get Started"
                </button>
                <button class="ml-4 inline-flex text-indigo-600 bg-indigo-100 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-200 rounded text-lg font-semibold">
                  "Documentation"
                </button>
              </div>
            </div>
            <div class="lg:max-w-lg lg:w-full md:w-1/2 w-5/6">
              <img
                class="object-cover object-center rounded shadow-lg"
                alt="Rustify UI hero"
                src="https://dummyimage.com/720x600/4f46e5/ffffff&text=Rustify+Rust+UI"
              />
            </div>
          </div>
        </section>
    }
}

