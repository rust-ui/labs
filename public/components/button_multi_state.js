const STATES = {
  idle: "Do some hard work",
  working: "⏳ working...",
  done: "Done! ✅",
};

demo.onclick = () => {
  setState("working");
  setTimeout(() => setState("done"), 2000);
  setTimeout(() => setState("idle"), 4000);
};

function setState(state) {
  if (!document.startViewTransition) demo.innerHTML = STATES[state];
  else document.startViewTransition(() => (demo.innerHTML = STATES[state]));
}
