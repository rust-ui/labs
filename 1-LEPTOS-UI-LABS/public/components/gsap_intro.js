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

const SHARED_TRIGGER = {
  start: "top top",
  end: "+=200%",
  scrub: 1,
};

gsap.to(".header__scroll .letters:first-child", {
  x: () => -innerWidth * 3,
  scale: 10,
  ease: "power2.inOut",
  scrollTrigger: SHARED_TRIGGER,
});

gsap.to(".header__scroll .letters:last-child", {
  x: () => innerWidth * 3,
  scale: 10,
  ease: "power2.inOut",
  scrollTrigger: SHARED_TRIGGER,
});

gsap.to(".img__holder", {
  rotation: 0,
  clipPath: "polygon(0% 0%, 100% 0%, 100% 100%, 0% 100%)",
  ease: "power2.inOut",
  scrollTrigger: SHARED_TRIGGER,
});

gsap.to(".img__holder img", {
  scale: 1,
  ease: "power2.inOut",
  scrollTrigger: SHARED_TRIGGER,
});
