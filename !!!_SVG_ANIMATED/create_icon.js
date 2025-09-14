async function createIcon(ComponentName, icon_filename) {
  // Load SVG paths
  const response = await fetch(`ICONS_WIP/${icon_filename}.txt`);
  const pathsHtml = await response.text();

  // Construct CSS filename from icon_filename
  const css_filename = `${icon_filename}_animate`;

  // Load and inject CSS
  const cssResponse = await fetch(`ICONS_WIP/${css_filename}.txt`);
  const cssContent = await cssResponse.text();

  // Check if style element for this CSS already exists
  const existingStyle = document.querySelector(`style[data-css="${css_filename}"]`);
  if (!existingStyle) {
    const style = document.createElement("style");
    style.setAttribute("data-css", css_filename);
    style.textContent = cssContent.trim();
    document.head.appendChild(style);
  }

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
