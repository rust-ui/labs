use leptos::prelude::*;
use leptos_meta::Stylesheet;

// TODO. On the HTML demo, it closes on click outside.

#[component]
pub fn DemoSheetCss() -> impl IntoView {
    view! {
        <Stylesheet href="/components/sheet_css.css" />
        <script src="/components/sheet_css.js"></script>

        <div class="mainDiv">
            <button onclick="demo.showModal()">Show Side Dialog</button>

            <dialog id="demo" class="slide-out">
                <section>
                    <header>
                        <button
                            title="Close"
                            onclick="demo.close()"
                            class="material-symbols-outlined"
                        >
                            close
                        </button>

                        <p>
                            "Lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam, quos."
                        </p>
                        <p>
                            "Lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam, quos."
                        </p>
                    </header>
                </section>
            </dialog>
        </div>
    }
}
