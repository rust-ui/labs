(() => {
  const applyMouseTracking = () => {
    const cards = document.querySelectorAll(".card");
    cards.forEach((card) => {
      card.addEventListener("mousemove", (e) => {
        const rect = card.getBoundingClientRect();
        const x = ((e.clientX - rect.left) / rect.width) * 100;
        const y = ((e.clientY - rect.top) / rect.height) * 100;
        card.style.setProperty("--x", `${x}%`);
        card.style.setProperty("--y", `${y}%`);
      });
    });
  };

  // For SSR, wait until content is loaded
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", applyMouseTracking);
  } else {
    applyMouseTracking();
  }

  // For CSR/hydration, run again after Leptos has hydrated
  if (window._$LEPTOS) {
    window._$LEPTOS.on("leptos:end", applyMouseTracking);
  }
})();
