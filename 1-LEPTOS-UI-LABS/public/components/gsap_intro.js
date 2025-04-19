gsap.registerPlugin(ScrollTrigger);

document.addEventListener("DOMContentLoaded", () => {
  const contentHolderHeight = document.querySelector(".content__holder").offsetHeight;
  const imgHolderHeight = window.innerHeight;
  const additionalScrollHeight = window.innerHeight;

  const totalBodyHeight = contentHolderHeight + imgHolderHeight + additionalScrollHeight;
  document.body.style.height = `${totalBodyHeight}px`;
});

ScrollTrigger.create({
  trigger: ".fixed__content",
  start: "-0.1% top",
  end: "bottom bottom",
  onEnter: () => {
    gsap.set(".fixed__content", { position: "absolute", top: "195%" });
  },
  onLeaveBack: () => {
    gsap.set(".fixed__content", { position: "fixed", top: "0" });
  },
});

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

const animateLetters = (selector, options = {}) => {
  const { scale = 1, xOffset = 0, customProps = {} } = options;

  gsap.to(selector, {
    ...(xOffset && { x: () => xOffset * innerWidth }),
    scale,
    ease: "power2.inOut",
    scrollTrigger: {
      start: "top top",
      end: "+=200%",
      scrub: 1,
    },
    ...customProps,
  });
};

// Options pattern
animateLetters(".header__scroll .letters:first-child", { scale: 10, xOffset: -3 });
animateLetters(".header__scroll .letters:last-child", { scale: 10, xOffset: 3 });
animateLetters(".img__holder img", { scale: 1 });
animateLetters(".img__holder", {
  customProps: {
    rotation: 0,
    clipPath: "polygon(0% 0%, 100% 0%, 100% 100%, 0% 100%)",
  },
});
