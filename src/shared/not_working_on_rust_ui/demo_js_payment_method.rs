use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoJsPaymentMethod() -> impl IntoView {
    view! {
        <Stylesheet id="payment-card" href="/components/payment_card.css" />
        <script src="/components/payment_card.js" />

        <main>
            <div class="payment-container">
                <div class="header">
                    <div>
                        <h2>Payment method</h2>
                        <div class="subtitle">Change how you pay for your purchases.</div>
                    </div>
                    <div class="button-container">
                        <button class="add-card-btn">
                            <svg width="16" height="16" viewBox="0 0 16 16">
                                <path
                                    d="M8 2v12M2 8h12"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                />
                            </svg>
                            <span>Add card</span>
                        </button>
                    </div>
                </div>

                <div class="container">
                    <div class="card-list">
                        <div
                            class="card-item"
                            data-index="1"
                            style="view-transition-name: card-item-edit1;"
                        >
                            <div class="card-item-content">
                                <div class="card-info">
                                    <img
                                        src="https://payment-method-lndev.vercel.app/visa.png"
                                        alt="Visa"
                                        class="card-logo"
                                        style="view-transition-name: card-logo1;"
                                    />
                                    <div>
                                        <input
                                            type="password"
                                            value="5211300256188330"
                                            class="card-number"
                                            style="view-transition-name: card-number1;"
                                        />
                                        <input
                                            type="text"
                                            value="12/22"
                                            class="card-expiry"
                                            style="view-transition-name: card-expiry1;"
                                        />
                                    </div>
                                </div>
                                <button
                                    class="edit-btn"
                                    type="button"
                                    style="view-transition-name: edit-btn1;"
                                >
                                    Edit
                                </button>
                            </div>
                        </div>

                        <div
                            class="card-item"
                            data-index="2"
                            style="view-transition-name: card-item-edit2;"
                        >
                            <div class="card-item-content">
                                <div class="card-info">
                                    <img
                                        src="https://payment-method-lndev.vercel.app/mastercard.png"
                                        alt="Mastercard"
                                        class="card-logo"
                                        style="view-transition-name: card-logo2;"
                                    />
                                    <div>
                                        <input
                                            type="password"
                                            value="5211300256183941"
                                            class="card-number"
                                            style="view-transition-name: card-number2;"
                                        />
                                        <input
                                            type="text"
                                            value="10/27"
                                            class="card-expiry"
                                            style="view-transition-name: card-expiry2;"
                                        />
                                    </div>
                                </div>
                                <button
                                    class="edit-btn"
                                    type="button"
                                    style="view-transition-name: edit-btn2;"
                                >
                                    Edit
                                </button>
                            </div>
                        </div>

                        <div
                            class="card-item"
                            data-index="3"
                            style="view-transition-name: card-item-edit3;"
                        >
                            <div class="card-item-content">
                                <div class="card-info">
                                    <img
                                        src="https://payment-method-lndev.vercel.app/visa.png"
                                        alt="Visa"
                                        class="card-logo"
                                        style="view-transition-name: card-logo3;"
                                    />
                                    <div>
                                        <input
                                            type="password"
                                            value="5211300256183941"
                                            class="card-number"
                                            style="view-transition-name: card-number3;"
                                        />
                                        <input
                                            type="text"
                                            value="06/25"
                                            class="card-expiry"
                                            style="view-transition-name: card-expiry3;"
                                        />
                                    </div>
                                </div>
                                <button
                                    class="edit-btn"
                                    type="button"
                                    style="view-transition-name: edit-btn3;"
                                >
                                    Edit
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
