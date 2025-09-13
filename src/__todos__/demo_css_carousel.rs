use leptos::prelude::*;

#[component]
pub fn DemoCssCarousel() -> impl IntoView {
    view! {
        <style>
            {"
            /* TW: Step 1 ✔️ Step 2 ✔️ Step 3 ✔️ Step 4 ✔️ Step 5 ✔️ Step 6 ✔️ Final ✔️ */
            .carousel__container {
              scroll-behavior: smooth;
              scroll-snap-type: x mandatory;
              overscroll-behavior-x: contain;
              scroll-marker-group: after;
              anchor-name: --carousel;
            }

            .carousel__slide {
              scroll-snap-align: center;
            }

            .carousel__container::scroll-marker-group {
              position: absolute;
              position-anchor: --carousel;
              top: calc(anchor(bottom));
              display: flex;
              justify-content: center;
              gap: 1em;
              inset-inline: 0;
            }

            .carousel__slide::scroll-marker {
              content: '';
              inline-size: 14px;
              block-size: 14px;
              border-radius: 50%;
              border: 2px solid #000;
              background: #fff;
              color: green;
              transition: background-color 0.3s;
            }

            .carousel__slide::scroll-marker:hover {
              background-color: #eee;
            }

            .carousel__slide::scroll-marker:target-current {
              background: currentColor;
            }

            .carousel__container::scroll-button(*) {
              font-size: 2em;
              position: absolute;
              text-align: center;
              position-anchor: --carousel;
              inline-size: 44px;
              block-size: 44px;
              border-radius: 50%;
              background: #fff;
              cursor: pointer;
              border: 2px solid #000;
              transition: background-color 0.3s;
            }

            .carousel__container::scroll-button(*):hover {
              background-color: #eee;
            }

            .carousel__container::scroll-button(left) {
              content: '';
              background: url('data:image/svg+xml;utf8,<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\"><path d=\"M15.293 3.293 6.586 12l8.707 8.707 1.414-1.414L9.414 12l7.293-7.293-1.414-1.414z\"/></svg>') no-repeat center;
              right: calc(anchor(left) + 1em);
              top: calc(anchor(center));
            }

            .carousel__container::scroll-button(right) {
              content: '';
              background: url('data:image/svg+xml;utf8,<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\"><path d=\"M7.293 4.707 14.586 12l-7.293 7.293 1.414 1.414L17.414 12 8.707 3.293 7.293 4.707z\"/></svg>')
                no-repeat center;
              left: calc(anchor(right) + 1em);
              top: calc(anchor(center));
            }

            .carousel__container::scroll-button(*):disabled {
              opacity: 0.35;
              cursor: default;
            }
            "}
        </style>

        <div class="bg-[#fff8f0] mt-8">
            <ul class="carousel__container flex gap-16 p-4 w-[800px] h-[500px] mx-auto overflow-x-hidden">
                <li class="carousel__slide list-none flex-[0_0_100%] flex items-center justify-center border-2 border-black bg-gray-200">
                    <h2>"Slide 1"</h2>
                </li>
                <li class="carousel__slide list-none flex-[0_0_100%] flex items-center justify-center border-2 border-black bg-gray-200">
                    <h2>"Slide 2"</h2>
                </li>
                <li class="carousel__slide list-none flex-[0_0_100%] flex items-center justify-center border-2 border-black bg-gray-200">
                    <h2>"Slide 3"</h2>
                </li>
                <li class="carousel__slide list-none flex-[0_0_100%] flex items-center justify-center border-2 border-black bg-gray-200">
                    <h2>"Slide 4"</h2>
                </li>
            </ul>
        </div>
    }
}