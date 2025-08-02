use leptos::prelude::*;

#[component]
pub fn DemoBento3Transition() -> impl IntoView {
    view! {
        // Inline style block
        <style>
            {r#"
            input:checked + .bento__grid__item {
            grid-row: 1/-1;
            grid-column: 1;
            }
            
            ::view-transition-group(*) {
            animation-duration: 0.5s;
            animation-timing-function: cubic-bezier(0.47, 1.64, 0.41, 0.8);
            }
            
            .bento__grid__item[data-index="1"] {
            view-transition-name: box-1;
            }
            
            .bento__grid__item[data-index="2"] {
            view-transition-name: box-2;
            }
            
            .bento__grid__item[data-index="3"] {
            view-transition-name: box-3;
            }
            "#}
        </style>

        // Main container structure
        <div class="mainDivParent text-[6.25vmax] max-lg:text-[60px] box-border">
            <div class="flex justify-center items-center min-h-screen font-sans mainDiv text-[#222] bg-[#fff6de] text-[0.3rem]">
                <div class="grid gap-6 w-full max-w-screen-md bento__grid__container grid-cols-[2fr_1fr] grid-rows-[1fr_1fr] aspect-[16/9]">

                    <input
                        checked=true
                        type="radio"
                        name="bento"
                        value="box-0"
                        id="box-0"
                        class="hidden"
                    />
                    <label
                        class="bg-center bg-no-repeat bg-cover cursor-pointer bento__grid__item bg-white/[0.33] rounded-[0.2rem] shadow-[0_0_12px_rgba(0,0,0,0.33)]"
                        for="box-0"
                        data-index="1"
                        style="background-image: url('https://images.unsplash.com/photo-1614348532352-c5dd2ad60420?crop=entropy&cs=srgb&fm=jpg&ixid=M3wzMjM4NDZ8MHwxfHJhbmRvbXx8fHx8fHx8fDE2OTk5MzIwMTJ8&ixlib=rb-4.0.3&q=85');"
                        onclick="event.preventDefault();document.startViewTransition?document.startViewTransition(()=>this.control.checked=true):this.control.checked=true"
                    ></label>

                    <input type="radio" name="bento" value="box-1" id="box-1" class="hidden" />
                    <label
                        class="bg-center bg-no-repeat bg-cover cursor-pointer bento__grid__item bg-white/[0.33] rounded-[0.2rem] shadow-[0_0_12px_rgba(0,0,0,0.33)]"
                        for="box-1"
                        data-index="2"
                        style="background-image: url('https://images.unsplash.com/photo-1566681682641-af84728ee122?crop=entropy&cs=srgb&fm=jpg&ixid=M3wzMjM4NDZ8MHwxfHJhbmRvbXx8fHx8fHx8fDE2OTk0NTY1NjR8&ixlib=rb-4.0.3&q=85');"
                        onclick="event.preventDefault();document.startViewTransition?document.startViewTransition(()=>this.control.checked=true):this.control.checked=true"
                    ></label>

                    <input type="radio" name="bento" value="box-2" id="box-2" class="hidden" />
                    <label
                        class="bg-center bg-no-repeat bg-cover cursor-pointer bento__grid__item bg-white/[0.33] rounded-[0.2rem] shadow-[0_0_12px_rgba(0,0,0,0.33)]"
                        for="box-2"
                        data-index="3"
                        style="background-image: url('https://images.unsplash.com/photo-1616093266211-d98e19d1f5bc?crop=entropy&cs=srgb&fm=jpg&ixid=M3wzMjM4NDZ8MHwxfHJhbmRvbXx8fHx8fHx8fDE2OTk0NTY2MDd8&ixlib=rb-4.0.3&q=85');"
                        onclick="event.preventDefault();document.startViewTransition?document.startViewTransition(()=>this.control.checked=true):this.control.checked=true"
                    ></label>

                </div>
            </div>
        </div>
    }
}
