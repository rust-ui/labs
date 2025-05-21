document.querySelectorAll('.delete-btn').forEach(btn => {
  const card = btn.parentElement;
  btn.addEventListener('click', () => {
    if (document.startViewTransition) {
      document.startViewTransition(() => {
        card.remove();
      });
    } else {
      card.remove();
    }
  })
})