async function createIcons(ComponentName, filename) {
  const response = await fetch(`icons/${filename}.txt`);
  const pathsHtml = await response.text();

  const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.setAttribute("data-name", ComponentName);
  svg.className = "size-7 text-gray-700";
  svg.style.width = "1.75rem";
  svg.style.height = "1.75rem";
  svg.setAttribute("stroke", "currentColor");
  svg.setAttribute("fill", "none");
  svg.setAttribute("stroke-width", "2");
  svg.setAttribute("stroke-linecap", "round");
  svg.setAttribute("stroke-linejoin", "round");
  svg.setAttribute("viewBox", "0 0 24 24");

  svg.innerHTML = pathsHtml.trim();

  document.getElementById("icon__container").appendChild(svg);
}
