const PARENT = document.querySelector("#parent");
const CHILDREN = PARENT.querySelectorAll("*");
const BUTTON = document.querySelector("#nav-toggle");
const LINK_LI = document.querySelectorAll("li:has(a)");

BUTTON.addEventListener("click", (e) => {
  PARENT.classList.toggle("active");
});

LINK_LI.forEach((el) => {
  el.addEventListener("click", (e) => {
    LINK_LI.forEach((el2) => {
      if (el !== el2) {
        el2.classList.remove("active");
      }
    });
    el.classList.toggle("active");
  });
});

document.addEventListener("mouseover", (e) => {
  if (!Array.from(CHILDREN).includes(e.target) && e.target !== PARENT) {
    PARENT.classList.remove("active");
  }
});
