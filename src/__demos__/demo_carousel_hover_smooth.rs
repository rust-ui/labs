use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoCarouselHoverSmooth() -> impl IntoView {
    view! {
        <Stylesheet href="/components/carousel_hover_smooth.css" />

        <div class="mainDiv">
            <div class="my__container">
                <div class="indicator"></div>
                <div class="gallery">
                    <div class="gallery-item">
                        <img
                            src="https://images.unsplash.com/photo-1631679706909-1844bbd07221?q=80&w=2892&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item">
                        <img
                            src="https://images.unsplash.com/photo-1578683010236-d716f9a3f461?q=80&w=2940&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item">
                        <img
                            src="https://images.unsplash.com/photo-1436262513933-a0b06755c784?q=80&w=2942&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item">
                        <img
                            src="https://images.unsplash.com/photo-1516018648631-0a79b7ea609c?q=80&w=2942&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item">
                        <img
                            src="https://images.unsplash.com/photo-1497433550656-7fb185be365e?w=800&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1yZWxhdGVkfDYzfHx8ZW58MHx8fHx8"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item">
                        <img
                            src="https://images.unsplash.com/photo-1541499768294-44cad3c95755?q=80&w=2946&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item">
                        <img
                            src="https://images.unsplash.com/photo-1574784619102-f7e342f21aa0?q=80&w=2940&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item">
                        <img
                            src="https://images.unsplash.com/photo-1614283233556-f35b0c801ef1?q=80&w=2787&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item">
                        <img
                            src="https://scontent.fsyd5-1.fna.fbcdn.net/v/t39.30808-6/475309919_1243442080618666_3847989983795528945_n.jpg?_nc_cat=109&ccb=1-7&_nc_sid=833d8c&_nc_ohc=VTU0EMuRgoUQ7kNvgG0bwHz&_nc_zt=23&_nc_ht=scontent.fsyd5-1.fna&_nc_gid=AXz6v4X2K-b0VdiKU_PCCMR&oh=00_AYCqS8e-wRAykwgJ88nhNJV1NYcSEkv-0SE5sQB2lC2V9g&oe=67AA04DD"
                            alt=""
                        />
                    </div>

                    <div class="gallery-item">
                        <img
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
