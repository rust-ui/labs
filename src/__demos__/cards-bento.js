window.initCardsBento = function () {
  const positions = [
    [150, 0], // Card 1: top center
    [0, 170], // Card 2: middle left
    [150, 340], // Card 3: bottom center
  ];
  const curves = [
    [60, 90], // 1 -> 2
    [60, 260], // 2 -> 3
    [250, 170], // 3 -> 1
  ];
  let order = [0, 1, 2];

  function bezierMove(card, from, to, curve, duration = 1200, cb) {
    let start = null;
    function step(ts) {
      if (!start) start = ts;
      let t = (ts - start) / duration;
      if (t > 1) t = 1;
      t = t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2; // ease in-out cubic
      const x =
        (1 - t) * (1 - t) * from[0] +
        2 * (1 - t) * t * curve[0] +
        t * t * to[0];
      const y =
        (1 - t) * (1 - t) * from[1] +
        2 * (1 - t) * t * curve[1] +
        t * t * to[1];
      card.style.transform = `translate(${x - from[0]}px,${y - from[1]}px)`;
      if (t < 1) {
        requestAnimationFrame(step);
      } else if (cb) {
        cb();
      }
    }
    requestAnimationFrame(step);
  }

  function renderAllCards() {
    for (let i = 0; i < 3; i++) {
      const card = document.getElementById(`card${i + 1}`);
      const pos = positions[order.indexOf(i)];
      card.style.transform = `translate(0,0)`;
      card.style.left = pos[0] + "px";
      card.style.top = pos[1] + "px";
    }
  }

  renderAllCards();

  function animateClockwise() {
    const oldOrder = order.slice();
    order = [order[2], order[0], order[1]];
    for (let i = 0; i < 3; i++) {
      const cardEl = document.getElementById(`card${i + 1}`);
      const fromIdx = oldOrder.indexOf(i);
      const toIdx = order.indexOf(i);
      const from = positions[fromIdx];
      const to = positions[toIdx];
      let curve;
      if (fromIdx === 0 && toIdx === 1) curve = curves[0];
      else if (fromIdx === 1 && toIdx === 2) curve = curves[1];
      else if (fromIdx === 2 && toIdx === 0) curve = curves[2];
      else if (fromIdx === 1 && toIdx === 0) curve = [100, 90];
      else if (fromIdx === 2 && toIdx === 1) curve = [100, 260];
      else if (fromIdx === 0 && toIdx === 2) curve = [250, 260];
      else curve = [(from[0] + to[0]) / 2, (from[1] + to[1]) / 2];
      bezierMove(cardEl, from, to, curve, 1200, () => {
        cardEl.style.left = to[0] + "px";
        cardEl.style.top = to[1] + "px";
        cardEl.style.transform = "translate(0,0)";
      });
    }
  }

  ["card1", "card2", "card3"].forEach((id) => {
    document.getElementById(id).addEventListener("click", animateClockwise);
  });
};
