let navigation = document.querySelector(".navigation");
let close = document.querySelector(".close");
navigation.onclick = function () {
  navigation.classList.add("active");
};
close.onclick = function () {
  navigation.classList.remove("active");
};