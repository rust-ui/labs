(() => {
  const notify = {
    toggler: document.querySelector("[notify-toggler]"),
    wrapper: document.querySelector("[notify-wrapper]"),
    cancel: document.querySelector("[notify-cancel]"),
  };

  notify.toggler.removeAttribute("notify-toggler");
  notify.wrapper.removeAttribute("notify-wrapper");
  notify.cancel.removeAttribute("notify-cancel");

  notify.toggler.addEventListener("click", (event) => {
    notify.wrapper.classList.add("active");
    notify.wrapper.classList.remove("hidden");
    notify.wrapper.classList.add("flex");
  });

  notify.cancel.addEventListener("click", (event) => {
    notify.wrapper.classList.remove("active");
    notify.wrapper.classList.remove("flex");
    notify.wrapper.classList.add("hidden");
  });
})();
