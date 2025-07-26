use leptos::prelude::*;

#[component]
pub fn DemoCarouselHoverSmooth() -> impl IntoView {
    view! {
        <style>
            {r#"
.my__container {
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    transform-origin: center;
}

.indicator {
    transition: all 2s cubic-bezier(0.075, 0.082, 0.165, 1);
}

.gallery-item {
    flex: 0 1 20px;
    transition: all 2s cubic-bezier(0.075, 0.82, 0.165, 1);
}

.gallery-item img {
    transform: scale(2);
}
            "#}
        </style>

        <div class="w-screen h-[200vh] bg-black">
            <div class="my__container absolute flex justify-center overflow-hidden w-[70%] mx-auto mb-8 py-4 px-2">
                <div class="indicator absolute top-0 left-0 w-[5px] h-[5px] bg-white rounded-full"></div>
                <div class="gallery flex justify-around w-full overflow-hidden">
                    <div class="gallery-item flex justify-center items-center h-[500px] bg-black overflow-hidden mx-[5px]">
                        <img
                            class="w-[500px] h-full object-contain"
                            src="https://images.unsplash.com/photo-1631679706909-1844bbd07221?q=80&w=2892&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item flex justify-center items-center h-[500px] bg-black overflow-hidden mx-[5px]">
                        <img
                            class="w-[500px] h-full object-contain"
                            src="https://images.unsplash.com/photo-1578683010236-d716f9a3f461?q=80&w=2940&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item flex justify-center items-center h-[500px] bg-black overflow-hidden mx-[5px]">
                        <img
                            class="w-[500px] h-full object-contain"
                            src="https://images.unsplash.com/photo-1436262513933-a0b06755c784?q=80&w=2942&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item flex justify-center items-center h-[500px] bg-black overflow-hidden mx-[5px]">
                        <img
                            class="w-[500px] h-full object-contain"
                            src="https://images.unsplash.com/photo-1516018648631-0a79b7ea609c?q=80&w=2942&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item flex justify-center items-center h-[500px] bg-black overflow-hidden mx-[5px]">
                        <img
                            class="w-[500px] h-full object-contain"
                            src="https://images.unsplash.com/photo-1497433550656-7fb185be365e?w=800&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1yZWxhdGVkfDYzfHx8ZW58MHx8fHx8"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item flex justify-center items-center h-[500px] bg-black overflow-hidden mx-[5px]">
                        <img
                            class="w-[500px] h-full object-contain"
                            src="https://images.unsplash.com/photo-1541499768294-44cad3c95755?q=80&w=2946&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item flex justify-center items-center h-[500px] bg-black overflow-hidden mx-[5px]">
                        <img
                            class="w-[500px] h-full object-contain"
                            src="https://images.unsplash.com/photo-1574784619102-f7e342f21aa0?q=80&w=2940&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item flex justify-center items-center h-[500px] bg-black overflow-hidden mx-[5px]">
                        <img
                            class="w-[500px] h-full object-contain"
                            src="https://images.unsplash.com/photo-1614283233556-f35b0c801ef1?q=80&w=2787&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item flex justify-center items-center h-[500px] bg-black overflow-hidden mx-[5px]">
                        <img
                            class="w-[500px] h-full object-contain"
                            src="https://scontent.fsyd5-1.fna.fbcdn.net/v/t39.30808-6/475309919_1243442080618666_3847989983795528945_n.jpg?_nc_cat=109&ccb=1-7&_nc_sid=833d8c&_nc_ohc=VTU0EMuRgoUQ7kNvgG0bwHz&_nc_zt=23&_nc_ht=scontent.fsyd5-1.fna&_nc_gid=AXz6v4X2K-b0VdiKU_PCCMR&oh=00_AYCqS8e-wRAykwgJ88nhNJV1NYcSEkv-0SE5sQB2lC2V9g&oe=67AA04DD"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item flex justify-center items-center h-[500px] bg-black overflow-hidden mx-[5px]">
                        <img
                            class="w-[500px] h-full object-contain"
                            src="https://images.unsplash.com/photo-1623606798652-c1c904e87e26?q=80&w=2940&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                </div>
            </div>
        </div>

        <script src="/components/carousel_hover_smooth.js"></script>
    }
}
