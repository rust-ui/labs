grid.onclick = () => {
  if (document.startViewTransition) {
    document.startViewTransition(() => {
      grid.classList.toggle("open");
    });
  } else {
    grid.classList.toggle("open");
  }
};
