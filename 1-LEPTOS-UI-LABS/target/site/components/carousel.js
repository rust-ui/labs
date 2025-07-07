const carousel = document.querySelector(".carousel");
const prevButton = document.querySelector(".carousel-control-prev");
const nextButton = document.querySelector(".carousel-control-next");

prevButton.addEventListener("click", () => {
  carousel.scrollBy({
    left: -carousel.offsetWidth,
    behavior: "smooth",
  });
});

nextButton.addEventListener("click", () => {
  carousel.scrollBy({
    left: carousel.offsetWidth,
    behavior: "smooth",
  });
});
