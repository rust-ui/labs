async function createIcon(ComponentName, icon_filename) {
  // Load SVG paths
  const response = await fetch(`ICONS_WIP/${icon_filename}.txt`);
  const pathsHtml = await response.text();

  // Load and inject CSS
  const cssResponse = await fetch(`ICONS_WIP/${icon_filename}.css`);
  const cssContent = await cssResponse.text();

  // Check if style element for this CSS already exists
  const existingStyle = document.querySelector(`style[data-css="${icon_filename}"]`);
  if (!existingStyle) {
    const style = document.createElement("style");
    style.setAttribute("data-css", icon_filename);
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

  // Create wrapper div with hover styles
  const wrapper = document.createElement("div");
  wrapper.className =
    "group flex justify-center items-center p-2 rounded-md transition-colors duration-200 cursor-pointer select-none hover:bg-gray-100";

  // Append SVG to wrapper, then wrapper to container
  // wrapper.appendChild(svg);
  // document.getElementById("icon__container").appendChild(wrapper);
  document.getElementById("icon__container").appendChild(svg);
}
