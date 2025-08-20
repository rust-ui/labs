use leptos::prelude::*;
use leptos_ui::{clx, div, img};

#[component]
pub fn DemoCarouselV2() -> impl IntoView {
    clx! {CarouselContainer, div, "relative"}
    clx! {CarouselSlide, div, "carousel__slide", "relative"}
    clx! {CarouselCaption, div, "absolute bottom-0 py-3 px-5 w-full text-center text-white bg-black/40"}
    clx! {CarouselController, div, "absolute top-1/2 p-4 text-white -translate-y-1/2 cursor-pointer hover:text-amber-500 bg-black/30 hover:bg-black/50"}
    clx! {CarouselDots, div, "flex justify-center items-center space-x-5"}
    div! {CarouselDot, "carousel__dot", "size-4 rounded-full cursor-pointer"}
    img! {CarouselImage, "object-cover w-full"}

    view! {
        <CarouselContainer class="mx-auto w-[600px]">
            <CarouselSlide>
                <CarouselImage class="h-[300px]" src=IMAGE_1 />
                <CarouselCaption>{"Slide 1"}</CarouselCaption>
            </CarouselSlide>
            <CarouselSlide>
                <CarouselImage class="h-[300px]" src=IMAGE_2 />
                <CarouselCaption>{"Slide 2"}</CarouselCaption>
            </CarouselSlide>
            <CarouselSlide>
                <CarouselImage class="h-[300px]" src=IMAGE_3 />
                <CarouselCaption>{"Slide 2"}</CarouselCaption>
            </CarouselSlide>

            <CarouselController class="left-0" onclick="moveSlide(-1)">
                "❮"
            </CarouselController>
            <CarouselController class="right-0" onclick="moveSlide(1)">
                "❯"
            </CarouselController>
        </CarouselContainer>

        <br />

        <CarouselDots>
            <CarouselDot onclick="currentSlide(1)" />
            <CarouselDot onclick="currentSlide(2)" />
            <CarouselDot onclick="currentSlide(3)" />
        </CarouselDots>

        <script>{SCRIPT_CAROUSEL}</script>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const IMAGE_1: &str = "https://plus.unsplash.com/premium_photo-1664474619075-644dd191935f?q=80&w=1469&auto=format&fit=crop";
const IMAGE_2: &str =
    "https://images.unsplash.com/photo-1584395630827-860eee694d7b?q=80&w=1469&auto=format&fit=crop";
const IMAGE_3: &str =
    "https://images.unsplash.com/photo-1506744038136-46273834b3fb?q=80&w=1469&auto=format&fit=crop";

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                      ✨ CONSTANTS ✨                       */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const SCRIPT_CAROUSEL: &str = r#"
// set the default active slide to the first one
let slideIndex = 1;
showSlide(slideIndex);

// change slide with the prev/next button
function moveSlide(moveStep) {
    showSlide(slideIndex += moveStep);
}

// change slide with the dots
function currentSlide(n) {
    showSlide(slideIndex = n);
}

function showSlide(n) {
    let i;
    const slides = document.getElementsByClassName("carousel__slide");
    const dots = document.getElementsByClassName('carousel__dot');
    
    if (n > slides.length) { slideIndex = 1 }
    if (n < 1) { slideIndex = slides.length }

    // hide all slides
    for (i = 0; i < slides.length; i++) {
        slides[i].classList.add('hidden');
    }

    // remove active status from all dots
    for (i = 0; i < dots.length; i++) {
        dots[i].classList.remove('bg-yellow-500');
        dots[i].classList.add('bg-green-600');
    }

    // show the active slide
    slides[slideIndex - 1].classList.remove('hidden');

    // highlight the active dot
    dots[slideIndex - 1].classList.remove('bg-green-600');
    dots[slideIndex - 1].classList.add('bg-yellow-500');
}
"#;
