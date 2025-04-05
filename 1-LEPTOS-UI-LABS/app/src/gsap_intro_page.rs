use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn GsapIntroPage() -> impl IntoView {
    view! {
        <Stylesheet href="/components/gsap_intro.css" />

        <div class="h-full mainDiv overscroll-none">

            <div class="fixed flex flex-row w-full transform -translate-x-1/2 -translate-y-1/2 top-1/2 left-1/2 header__scroll z-2">
                <div class="flex flex-1 letters">
                    <div>"T"</div>
                    <div>"A"</div>
                    <div>"I"</div>
                    <div>"L"</div>
                </div>
                <div class="flex flex-1 letters">
                    <div>"W"</div>
                    <div>"I"</div>
                    <div>"N"</div>
                    <div>"D"</div>
                </div>
            </div>
            <div class="fixed top-0 w-full min-h-screen website-content">
                <div class="img__holder relative w-full h-[100vh] top-0 bg-white">
                    <img
                        class="object-cover w-full h-full"
                        src="https://images.unsplash.com/photo-1700403322391-f1cd144394cb?q=80&w=2070&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                        alt=""
                    />
                </div>

                <div class="relative p-1 text-white bg-black -w-full content__holder">
                    <div class="row">
                        <h1 class="font-normal tracking-tight uppercase text-8xl">History</h1>
                    </div>
                    <div class="row">
                        <div class=" w-[200px] h-[275px]">"Text"</div>
                    </div>

                    <div class="row">
                        <p class="w-1/2 text-base font-normal leading-relaxed ">
                            In the age of knights and castles, master sculptors skillfully
                            carved stones and marbles with great artistry. From mountains and
                            forests, the raw materials arose to be shaped according to their
                            vision.
                        </p>
                        <p>
                            In the age of knights and castles, master sculptors skillfully
                            carved stones and marbles with great artistry. From mountains and
                            forests, the raw materials arose to be shaped according to their
                            vision.
                        </p>
                        <p>
                            In the age of knights and castles, master sculptors skillfully
                            carved stones and marbles with great artistry. From mountains and
                            forests, the raw materials arose to be shaped according to their
                            vision.
                        </p>
                        <p>
                            In the age of knights and castles, master sculptors skillfully
                            carved stones and marbles with great artistry. From mountains and
                            forests, the raw materials arose to be shaped according to their
                            vision.
                        </p>
                        <p>
                            In the age of knights and castles, master sculptors skillfully
                            carved stones and marbles with great artistry. From mountains and
                            forests, the raw materials arose to be shaped according to their
                            vision.
                        </p>
                    </div>
                    <div class="row">
                        <p>
                            These artisans, wielding hammers and chisels, brought to life the
                            mysteries of faith and ancient tales in stone. Every stroke, with
                            precision and reverence, narrated stories of virtues, triumphs, and
                            sometimes tragic human tales.
                        </p>
                    </div>
                </div>
            </div>

        </div>

        <script src="/components/gsap_intro.js" />
    }
}
