use leptos::prelude::*;
use leptos_meta::Stylesheet;

// TODO. Replace the CSS of "/components/password_strength_meter.css" with Tailwind CSS.

#[component]
pub fn DemoPasswordStrengthMeter() -> impl IntoView {
    view! {
        <Stylesheet href="/components/password_strength_meter.css" />
        <Stylesheet href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" />

        <div class="password-strength-container">
            <div class="password-input-group">
                <input
                    type="password"
                    class="password-input"
                    id="password"
                    placeholder="Enter password"
                />
                <button type="button" class="password-toggle" id="toggle-password">
                    <i class="fas fa-eye"></i>
                </button>
            </div>

            <div class="strength-meter">
                <div class="strength-meter-fill" id="strength-fill"></div>
            </div>

            <div class="strength-text" id="strength-text">
                Enter a password
            </div>

            <div class="strength-requirements">
                <div class="requirement" id="length">
                    <i class="fas fa-times-circle"></i>
                    At least 8 characters
                </div>
                <div class="requirement" id="uppercase">
                    <i class="fas fa-times-circle"></i>
                    At least 1 uppercase letter
                </div>
                <div class="requirement" id="lowercase">
                    <i class="fas fa-times-circle"></i>
                    At least 1 lowercase letter
                </div>
                <div class="requirement" id="number">
                    <i class="fas fa-times-circle"></i>
                    At least 1 number
                </div>
                <div class="requirement" id="special">
                    <i class="fas fa-times-circle"></i>
                    At least 1 special character
                </div>
            </div>
        </div>

        <script src="/components/password_strength_meter.js" />
    }
}
