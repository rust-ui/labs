use leptos::prelude::*;

#[component]
pub fn DemoChips() -> impl IntoView {
    view! {
        <style>
            {"
             :root {
               --accent-color: orange;
             }
            
             [data-name=\"ChipsContainer\"] label::after {
               content:\"\";
               background:
                 linear-gradient(230deg, var(--accent-color) 0 0.96vmin, #fff0 0 100%),
                 linear-gradient(142deg, var(--accent-color) 0 1.12vmin, #fff0 0 100%),
                 conic-gradient(from 43deg at 43% calc(64% + 0.24vmin), #fff0 0 0%, var(--accent-color) 1% 76%, #fff0 77% 100%),
                 conic-gradient(from -45deg at 43% 64%, #fff0 0 0%, var(--accent-color) 2% 25%, #fff0 26% 100%);
             }
            "}
        </style>

        <div class="flex overflow-hidden flex-col justify-center items-center p-0 m-0 w-screen h-screen bg-black mainDiv box-border">
            <div
                data-name="ChipsContainer"
                class="flex relative flex-wrap justify-start content-center w-full transition-all chips__main__container max-w-[66vmin] z-[1] bg-black/[0.02] p-[2.8vmin] rounded-[2vmin] shadow-[inset_0_0_1px_1px_#ebebeb] duration-[350ms] ease-[cubic-bezier(0.68,-0.55,0.265,1.55)] *:transition-all *:duration-[350ms] *:ease-[cubic-bezier(0.68,-0.55,0.265,1.55)] *:before:transition-all *:before:duration-[350ms] *:before:ease-[cubic-bezier(0.68,-0.55,0.265,1.55)] *:after:transition-all *:after:duration-[350ms] *:after:ease-[cubic-bezier(0.68,-0.55,0.265,1.55)]"
            >
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"mountain"</span>
                    <input type="checkbox" id="mountain" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"ocean"</span>
                    <input type="checkbox" id="ocean" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"forest"</span>
                    <input type="checkbox" id="forest" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"river"</span>
                    <input type="checkbox" id="river" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"desert"</span>
                    <input type="checkbox" id="desert" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"jungle"</span>
                    <input type="checkbox" id="jungle" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"beach"</span>
                    <input type="checkbox" id="beach" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"tropical"</span>
                    <input type="checkbox" id="tropical" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"tundra"</span>
                    <input type="checkbox" id="tundra" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"artic"</span>
                    <input type="checkbox" id="artic" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"lake"</span>
                    <input type="checkbox" id="lake" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"savanna"</span>
                    <input type="checkbox" id="savanna" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"snow"</span>
                    <input type="checkbox" id="snow" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"grass"</span>
                    <input type="checkbox" id="grass" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"taiga"</span>
                    <input type="checkbox" id="taiga" class="hidden" />
                </label>
                <label class="flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]">
                    <span>"pond"</span>
                    <input type="checkbox" id="pond" class="hidden" />
                </label>
            </div>
        </div>
    }
}
