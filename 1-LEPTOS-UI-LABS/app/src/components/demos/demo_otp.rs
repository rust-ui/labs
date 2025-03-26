use leptos::prelude::*;

#[component]
pub fn DemoOtp() -> impl IntoView {
    view! {
        <script src="/components/otp.js" />

        <div class="flex flex-col items-center justify-center h-screen space-y-5">
            <h1 class="text-3xl font-bold text-green-500">Submit OTP</h1>
            <div class="flex items-center gap-2.5 justify-center" id="otp-container">
                <input
                    type="text"
                    maxlength="1"
                    class="w-16 h-16 text-xl font-semibold text-center text-gray-700 transition-all bg-gray-100 border border-gray-300 rounded-md outline-none focus:border-blue-500"
                />
                <input
                    type="text"
                    maxlength="1"
                    class="w-16 h-16 text-xl font-semibold text-center text-gray-700 transition-all bg-gray-100 border border-gray-300 rounded-md outline-none focus:border-blue-500"
                />
                <input
                    type="text"
                    maxlength="1"
                    class="w-16 h-16 text-xl font-semibold text-center text-gray-700 transition-all bg-gray-100 border border-gray-300 rounded-md outline-none focus:border-blue-500"
                />
                <input
                    type="text"
                    maxlength="1"
                    class="w-16 h-16 text-xl font-semibold text-center text-gray-700 transition-all bg-gray-100 border border-gray-300 rounded-md outline-none focus:border-blue-500"
                />
                <input
                    type="text"
                    maxlength="1"
                    class="w-16 h-16 text-xl font-semibold text-center text-gray-700 transition-all bg-gray-100 border border-gray-300 rounded-md outline-none focus:border-blue-500"
                />
                <input
                    type="text"
                    maxlength="1"
                    class="w-16 h-16 text-xl font-semibold text-center text-gray-700 transition-all bg-gray-100 border border-gray-300 rounded-md outline-none focus:border-blue-500"
                />
            </div>
        </div>
    }
}
