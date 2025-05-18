use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoBottomBarAwwwards() -> impl IntoView {
    view! {
        <Stylesheet href="/components/bottom_bar_awwards.css" />

        <div class="mainDiv">
            <div id="parent">
                <div id="nav-top">
                    <div class="column">
                        <div class="heading">
                            <li>Awards</li>
                        </div>
                        <ul>
                            <li>
                                <a href="#">Winners</a>
                            </li>
                            <li>
                                <a href="#">Nominees</a>
                            </li>
                            <li>
                                <a href="#">Sites of the Day</a>
                            </li>
                            <li>
                                <a href="#">Sites of the Month</a>
                            </li>
                        </ul>
                    </div>
                    <div class="column">
                        <div class="heading">
                            <li>Inspiration</li>
                        </div>
                        <ul>
                            <li>
                                <a href="#">Collections</a>
                            </li>
                            <li>
                                <a href="#">Elements</a>
                            </li>
                        </ul>
                        <div class="heading">
                            <li>Blog</li>
                        </div>
                        <ul>
                            <li>
                                <a href="#">Articles</a>
                            </li>
                        </ul>
                    </div>
                    <div class="column">
                        <div class="heading">
                            <li>Directory</li>
                        </div>
                        <ul>
                            <li>
                                <a href="#">Professionals</a>
                            </li>
                            <li>
                                <a href="#">Freelancers</a>
                            </li>
                            <li>
                                <a href="#">Agencies & Studios</a>
                            </li>
                        </ul>
                    </div>
                    <div class="column">
                        <div class="heading">
                            <li>Academy</li>
                        </div>
                        <ul>
                            <li>
                                <a href="#">Courses</a>
                            </li>
                        </ul>
                        <div class="heading">
                            <li>Jobs & Jury</li>
                        </div>
                        <ul>
                            <li>
                                <a href="#">Jobs</a>
                            </li>
                            <li>
                                <a href="#">Jury</a>
                            </li>
                        </ul>
                    </div>
                </div>
                <div id="nav-bottom">
                    <div class="text">w. <span>home</span></div>
                    <button id="nav-toggle">
                        <div class="open">"📂"</div>
                        <div class="close">"❌"</div>
                    </button>
                </div>
            </div>
            <div id="offset"></div>

        </div>

        <script src="/components/bottom_bar_awwards.js"></script>
    }
}
