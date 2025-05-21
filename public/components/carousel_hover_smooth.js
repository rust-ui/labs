const galleryContainer = document.querySelector(".gallery");
const galleryItems = galleryContainer.querySelectorAll(".gallery-item");
const indicator = document.querySelector(".indicator");

const defaultItemFlex = "0 1 20px";
const hoverItemFlex = "1 1 400px";

const updateGalleryItems = () => {
  galleryItems.forEach((item) => {
    let flex = defaultItemFlex;

    if (item.isHovered) {
      flex = hoverItemFlex;
    }

    item.style.flex = flex;
  });
};

// Initialize the first item as hovered
galleryItems[0].isHovered = true;
updateGalleryItems();

// Add hover event listeners
galleryItems.forEach((item) => {
  item.addEventListener("mouseenter", () => {
    galleryItems.forEach((otheritem) => {
      otheritem.isHovered = otheritem === item;
    });

    updateGalleryItems();
  });
});

// Move indicator with mouse
galleryContainer.addEventListener("mousemove", (e) => {
  indicator.style.left = `${e.clientX - galleryContainer.getBoundingClientRect().left}px`;
});
