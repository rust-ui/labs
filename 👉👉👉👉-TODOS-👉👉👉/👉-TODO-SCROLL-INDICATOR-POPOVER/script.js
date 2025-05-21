import { Pane } from "https://cdn.skypack.dev/tweakpane@4.0.4";

const popover = document.querySelector("[popover]");

const config = {
  alignment: "center",
  theme: "light",
};

const ctrl = new Pane({
  title: "Config",
  expanded: false,
});

const update = () => {
  document.documentElement.dataset.theme = config.theme;
  document.documentElement.dataset.alignment = config.alignment;
};

const sync = (event) => {
  if (
    !document.startViewTransition ||
    (event.target.controller.view.labelElement.innerText !== "Theme" &&
      event.target.controller.view.labelElement.innerText !== "Alignment")
  )
    return update();
  document.startViewTransition(() => update());
};

ctrl.addBinding(config, "alignment", {
  label: "Alignment",
  options: {
    Left: "left",
    Center: "center",
    Right: "right",
  },
});

ctrl.addBinding(config, "theme", {
  label: "Theme",
  options: {
    System: "system",
    Light: "light",
    Dark: "dark",
  },
});

ctrl.on("change", sync);
update();

const links = document.querySelectorAll("ol a");
links.forEach((link) => {
  link.addEventListener("click", () => popover.hidePopover());
});

// Backfill the popover width/height transition
if (!CSS.supports("interpolate-size: allow-keywords")) {
  let set = false;
  popover.style.setProperty("transition", "none");
  popover.addEventListener("toggle", () => {
    if (!set) {
      const { height, width } = popover.getBoundingClientRect();
      document.documentElement.style.setProperty("--content-height", `${height}px`);
      document.documentElement.style.setProperty("--content-width", `${width}px`);
      set = true;
      popover.hidePopover();
      requestAnimationFrame(() => {
        popover.showPopover();
        popover.style.removeProperty("transition");
      });
    }
  });
}
