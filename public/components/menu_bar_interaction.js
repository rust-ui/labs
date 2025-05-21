const chatbar = document.getElementById("js-chatbar");

function toggle() {
  if (chatbar.classList.contains("--is-active")) {
    chatbar.classList.add("--drop");
    chatbar.classList.add("--drop");
    chatbar.classList.remove("--is-active");
    setTimeout(() => {
      chatbar.classList.remove("--drop");
    }, 600);
  } else {
    chatbar.classList.add("--bop");
    chatbar.classList.add("--is-active");
    setTimeout(() => {
      chatbar.classList.remove("--bop");
    }, 600);
  }
}
