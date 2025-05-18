// 🦔 ref: https://codepen.io/argyleink/pen/BaGrXmv

for (const radio of document.querySelectorAll('input[type="radio"]')) {
  radio.onclick = (e) => {
    if (!document.startViewTransition) return;

    e.preventDefault();
    document.startViewTransition(() => {
      e.target.checked = true;
    });
  };
}
