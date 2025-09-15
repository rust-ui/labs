// Custom tooltip functionality
function createTooltip() {
  const tooltip = document.createElement("div");
  tooltip.id = "custom-tooltip";
  tooltip.style.cssText = `
    position: absolute;
    background: transparent;
    color: #374151;
    padding: 3px 6px;
    border-radius: 3px;
    font-size: 11px;
    font-family: system-ui, -apple-system, sans-serif;
    pointer-events: none;
    z-index: 1000;
    opacity: 0;
    transition: opacity 0.3s ease;
    white-space: nowrap;
  `;
  document.body.appendChild(tooltip);
  return tooltip;
}

function showTooltip(event, text) {
  let tooltip = document.getElementById("custom-tooltip");
  if (!tooltip) {
    tooltip = createTooltip();
  }

  tooltip.textContent = text;
  tooltip.style.opacity = "1";

  // Position relative to mouse
  const rect = event.target.getBoundingClientRect();
  tooltip.style.left = `${rect.left + rect.width / 2 - tooltip.offsetWidth / 2}px`;
  tooltip.style.top = `${rect.top - tooltip.offsetHeight - 8}px`;
}

function hideTooltip() {
  const tooltip = document.getElementById("custom-tooltip");
  if (tooltip) {
    tooltip.style.opacity = "0";
  }
}

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
  svg.className = "size-10 text-gray-700";
  svg.style.width = "2.5rem";
  svg.style.height = "2.5rem";
  svg.setAttribute("stroke", "currentColor");
  svg.setAttribute("fill", "none");
  svg.setAttribute("stroke-width", "2");
  svg.setAttribute("stroke-linecap", "round");
  svg.setAttribute("stroke-linejoin", "round");
  svg.setAttribute("viewBox", "0 0 24 24");

  // Add custom tooltip events
  svg.addEventListener("mouseenter", (e) => showTooltip(e, icon_filename));
  svg.addEventListener("mouseleave", hideTooltip);

  svg.innerHTML = pathsHtml.trim();

  document.getElementById("icon__container").appendChild(svg);
}
