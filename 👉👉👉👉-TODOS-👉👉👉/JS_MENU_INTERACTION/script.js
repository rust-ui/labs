const widget = document.querySelector(".menu__widget");
const toggle = document.querySelector(".menu__toggle__button");

toggle.addEventListener("click", () => {
  widget.classList.toggle("active");
});
