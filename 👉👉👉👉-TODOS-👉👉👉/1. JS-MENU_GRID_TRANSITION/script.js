const navigation = document.querySelector(".grid__menu__navigation");
const close = document.querySelector(".grid__menu__close");
navigation.onclick = () => {
  navigation.classList.add("active");
};
close.onclick = () => {
  navigation.classList.remove("active");
};
