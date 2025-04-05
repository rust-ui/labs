document.addEventListener('DOMContentLoaded', function() {
  const toggleButton = document.querySelector('.appearance');
  const body = document.body;
  const prefersDarkScheme = window.matchMedia("(prefers-color-scheme: dark)");

  // Load the saved theme from localStorage or detect the user's preference
  const savedTheme = localStorage.getItem("theme");

  if (savedTheme) {
    body.classList.toggle("dark", savedTheme === "dark");
    toggleButton.classList.toggle("switch", savedTheme === "dark");
  } else if (prefersDarkScheme.matches) {
    body.classList.add("dark");
    toggleButton.classList.add("switch");
  }

  // Toggle between day and night mode
  toggleButton.addEventListener("click", () => {
    body.classList.toggle("dark");
    toggleButton.classList.toggle("switch");

    // Save the current mode in localStorage
    if (body.classList.contains("dark")) {
      localStorage.setItem("theme", "dark");
    } else {
      localStorage.setItem("theme", "light");
    }
  });

  // Automatically switch if the user changes their system preferences
  prefersDarkScheme.addEventListener("change", (e) => {
    if (e.matches) {
      body.classList.add("dark");
      toggleButton.classList.add("switch");
      localStorage.setItem("theme", "dark");
    } else {
      body.classList.remove("dark");
      toggleButton.classList.remove("switch");
      localStorage.setItem("theme", "light");
    }
  });
});
