use leptos::prelude::*;

#[component]
pub fn DemoSplitButton() -> impl IntoView {
    view! {
        <style>
            {"
            .split__button__container.open__split__button > .split__button__menu {
            display: block;
            }
            "}
        </style>

        <div class="p-0 m-0 box-border">
            <div class="flex p-20 mx-auto border border-gray-500">
                <div class="inline-block relative mt-5 text-left split__button__container">
                    <button class="float-left relative px-3 m-0 text-sm no-underline border cursor-pointer outline-none split__button__action leading-[27px] bg-[#f2f2f2] border-[#e0e0e0] shadow-[1px_1px_2px_#e0e0e0] h-[30px] text-[#333] hover:bg-[#e0e0e0] active:bg-[#d3d3d3]">
                        "❖ Action"
                    </button>
                    <button class="float-left relative px-2 m-0 text-xs border border-l-0 cursor-pointer outline-none split__button__dropdown leading-[27px] bg-[#f2f2f2] border-[#e0e0e0] shadow-[1px_1px_2px_#e0e0e0] h-[30px] text-[#333] hover:bg-[#e0e0e0] active:bg-[#d3d3d3]">
                        "▼"
                    </button>
                    <ul class="hidden absolute right-0 text-sm list-none bg-clip-padding border split__button__menu py-[5px] mt-[2px] top-[29px] z-[1000] min-w-[160px] bg-[#f2f2f2] border-[#e0e0e0] shadow-[1px_1px_2px_#e0e0e0]">
                        <li>
                            <a
                                href="#"
                                class="block px-5 no-underline duration-200 ease-in-out split__button__menu__item py-[6px] transition-[background-color] text-[#444] hover:bg-[#d3d3d3]"
                            >
                                Item - 1
                            </a>
                        </li>
                        <li>
                            <a
                                href="#"
                                class="block px-5 no-underline duration-200 ease-in-out split__button__menu__item py-[6px] transition-[background-color] text-[#444] hover:bg-[#d3d3d3]"
                            >
                                Item - 2
                            </a>
                        </li>
                        <li>
                            <a
                                href="#"
                                class="block px-5 no-underline duration-200 ease-in-out split__button__menu__item py-[6px] transition-[background-color] text-[#444] hover:bg-[#d3d3d3]"
                            >
                                Long Item - 3
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </div>

        <script>
            {"
            document.addEventListener('DOMContentLoaded', function() {
            const splitBtnContainer = document.querySelector('.split__button__container');
            const dropdownButton = document.querySelector('.split__button__dropdown');
            
            dropdownButton.addEventListener('click', function() {
            if (!splitBtnContainer.classList.contains('open__split__button')) {
             splitBtnContainer.classList.add('open__split__button');
            }
            });
            
            splitBtnContainer.addEventListener('click', function(event) {
            event.stopPropagation();
            });
            
            document.addEventListener('click', function() {
            if (splitBtnContainer.classList.contains('open__split__button')) {
             splitBtnContainer.classList.remove('open__split__button');
            }
            });
            });
            "}
        </script>
    }
}
