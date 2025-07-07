(() => {
  const WORDS = ["Productivity", "Efficiency", "Passion", "Creativity"];

  let count = 0;

  const performSwap = () => {
    const SWAPPER = document.querySelector("span:nth-of-type(2)");
    if (!document.startViewTransition) {
      SWAPPER.innerText = WORDS[(count += 1) % WORDS.length];
    } else {
      document.startViewTransition(() => {
        SWAPPER.innerText = WORDS[(count += 1) % WORDS.length];
      });
    }
  };

  const performSwapEachInterval = () => {
    setInterval(performSwap, 2000);
  };

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", performSwapEachInterval);
  } else {
    performSwapEachInterval();
  }

  // for hydration
  if (window._$LEPTOS) {
    window._$LEPTOS.on("leptos:end", performSwapEachInterval);
  }
})();
