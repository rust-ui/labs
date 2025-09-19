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
                <ChipItem label="sunny" />
                <ChipItem label="cloudy" />
                <ChipItem label="rainy" />
                <ChipItem label="windy" />
                <ChipItem label="stormy" />
                <ChipItem label="foggy" />
                <ChipItem label="snowy" />
                <ChipItem label="icy" />
                <ChipItem label="humid" />
                <ChipItem label="dry" />
                <ChipItem label="hot" />
                <ChipItem label="cold" />
                <ChipItem label="warm" />
                <ChipItem label="chilly" />
                <ChipItem label="breezy" />
                <ChipItem label="gusty" />
                <ChipItem label="hazy" />
            </div>
        </div>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn ChipItem(#[prop(into)] label: &'static str) -> impl IntoView {
    const CHIP_LABEL_CLASS: &str = "flex overflow-hidden relative items-center w-auto cursor-pointer h-[6vmin] bg-neutral-600 rounded-[8vmin] py-[2vmin] pr-[2vmin] pl-[3vmin] text-[2vmin] text-[#ebebeb] m-[1vmin] shadow-[inset_0_0_0_1px_#ebebeb] has-[:checked]:shadow-[inset_0_0_0_2px_var(--accent-color)] has-[:checked]:text-[var(--accent-color)] after:content-[''] after:w-0 after:h-0 after:ml-[1vmin] after:rounded-[8vmin] after:text-[0px] has-[:checked]:after:w-[2vmin] has-[:checked]:after:h-[2vmin] has-[:checked]:after:overflow-hidden has-[:checked]:after:text-[22px]";

    view! {
        <label data-name="ChipItem" class=CHIP_LABEL_CLASS>
            <span>{label}</span>
            <input type="checkbox" id=label class="hidden" />
        </label>
    }
}
