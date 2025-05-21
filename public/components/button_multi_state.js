const states = {
    idle: "Do some hard work",
    sending:
      '<img aria-hidden="true" width="24" height="24" src="https://assets.codepen.io/2585/ring-resize.svg" alt="" /> working...',
    done: "Done!",
  };
  
  demo.onclick = () => {
    setState("sending");
    setTimeout(() => setState("done"), 4000);
    setTimeout(() => setState("idle"), 6000);
  };
  
  function setState(state) {
    if (!document.startViewTransition) demo.innerHTML = states[state];
    else document.startViewTransition(() => (demo.innerHTML = states[state]));
  }
  //# sourceURL=pen.js
  