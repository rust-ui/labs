const navigation = document.querySelector(".navigation");
const close = document.querySelector(".close");
navigation.onclick = () => {
  navigation.classList.add("active");
};
close.onclick = () => {
  navigation.classList.remove("active");
};
