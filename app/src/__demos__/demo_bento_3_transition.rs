use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoBento3Transition() -> impl IntoView {
    view! {
        <Stylesheet href="/components/bento_3_transition.css" />

        <div class="mainDivParent">
            <div class="mainDiv">
                <div class="wrapper">
                    <input checked type="radio" name="bento" value="box-0" id="box-0" />
                    <label
                        class="box"
                        for="box-0"
                        data-index="1"
                        style="background-image: url('https://images.unsplash.com/photo-1614348532352-c5dd2ad60420?crop=entropy&cs=srgb&fm=jpg&ixid=M3wzMjM4NDZ8MHwxfHJhbmRvbXx8fHx8fHx8fDE2OTk5MzIwMTJ8&ixlib=rb-4.0.3&q=85');"
                    ></label>

                    <input type="radio" name="bento" value="box-1" id="box-1" />
                    <label
                        class="box"
                        for="box-1"
                        data-index="2"
                        style="background-image: url('https://images.unsplash.com/photo-1566681682641-af84728ee122?crop=entropy&cs=srgb&fm=jpg&ixid=M3wzMjM4NDZ8MHwxfHJhbmRvbXx8fHx8fHx8fDE2OTk0NTY1NjR8&ixlib=rb-4.0.3&q=85');"
                    ></label>

                    <input type="radio" name="bento" value="box-2" id="box-2" />
                    <label
                        class="box"
                        for="box-2"
                        data-index="3"
                        style="background-image: url('https://images.unsplash.com/photo-1616093266211-d98e19d1f5bc?crop=entropy&cs=srgb&fm=jpg&ixid=M3wzMjM4NDZ8MHwxfHJhbmRvbXx8fHx8fHx8fDE2OTk0NTY2MDd8&ixlib=rb-4.0.3&q=85');"
                    ></label>
                </div>
            </div>
        </div>

        <script src="/components/bento_3_transition.js"></script>
    }
}
