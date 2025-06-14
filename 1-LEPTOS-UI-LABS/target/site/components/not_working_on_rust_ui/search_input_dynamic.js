const searchInput = document.getElementById("searchInput");
const searchContainer = searchInput.parentElement;
const searchIcon = searchContainer.querySelector("svg");

searchInput.addEventListener("focus", () => {
  searchContainer.classList.add("w-96", "bg-white");
  searchContainer.classList.remove("w-32");
  searchInput.placeholder = "Yazın ve Enter'a basın ↵";
  searchInput.classList.remove("border-gray-300");
  searchInput.classList.add("border-blue-500");
  searchIcon.classList.add("text-blue-500");
  searchIcon.classList.remove("text-gray-400");
});

searchInput.addEventListener("blur", () => {
  searchContainer.classList.add("w-32");
  searchContainer.classList.remove("w-96", "bg-white");
  searchInput.placeholder = "Arama..";
  searchInput.classList.add("border-gray-300");
  searchInput.classList.remove("border-blue-500");
  searchIcon.classList.add("text-gray-400");
  searchIcon.classList.remove("text-blue-500");
});
