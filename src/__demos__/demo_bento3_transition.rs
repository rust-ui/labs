use leptos::IntoView;
use leptos::prelude::{ClassAttribute, ElementChild, IntoMaybeErased, component, view};

use leptos::*;

#[component]
pub fn DemoBento3Transition() -> impl IntoView {
    view! {
    <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100 relative">
         <div class="relative w-[440px] h-[420px]">
             <div
                 id="card1"
                 class="w-40 h-32 bg-blue-500 rounded-xl shadow-xl flex items-center justify-center text-white text-2xl font-bold absolute cursor-pointer"
                 style="left: 150px; top: 0px;"
             >
                 Card 1
             </div>
             <div
                 id="card2"
                 class="w-40 h-32 bg-pink-500 rounded-xl shadow-xl flex items-center justify-center text-white text-2xl font-bold absolute cursor-pointer"
                 style="left: 0px; top: 170px;"
             >
                 Card 2
             </div>
             <div
                 id="card3"
                 class="w-40 h-32 bg-green-500 rounded-xl shadow-xl flex items-center justify-center text-white text-2xl font-bold absolute cursor-pointer"
                 style="left: 150px; top: 340px;"
             >
                 Card 3
             </div>
         </div>
         <script>
             {r#"
const positions = [
    [150, 0],    // Card 1: top center
    [0, 170],    // Card 2: middle left
    [150, 340],  // Card 3: bottom center
];

// Control points for smooth curves (quadratic bezier)
const curves = [
    [60, 90],    // 1 -> 2
    [60, 260],   // 2 -> 3
    [250, 170],  // 3 -> 1
];

let order = [0, 1, 2]; // Card1 at pos 0, Card2 at pos 1, Card3 at pos 2

function bezierMove(card, from, to, curve, duration = 1200, cb) {
    let start = null;
    function step(ts) {
        if (!start) start = ts;
        let t = (ts - start) / duration;
        if (t > 1) t = 1;
        // Ease in-out cubic for smoothness
        t = t < 0.5 ? 4*t*t*t : 1 - Math.pow(-2*t+2, 3)/2;
        const x = (1-t)*(1-t)*from[0] + 2*(1-t)*t*curve[0] + t*t*to[0];
        const y = (1-t)*(1-t)*from[1] + 2*(1-t)*t*curve[1] + t*t*to[1];
        card.style.transform = `translate(${x-from[0]}px,${y-from[1]}px)`;
        if (t < 1) {
            requestAnimationFrame(step);
        } else if (cb) {
            cb();
        }
    }
    requestAnimationFrame(step);
}

function renderAllCards() {
    for(let i=0;i<3;i++) {
        const card = document.getElementById(`card${i+1}`);
        const pos = positions[order.indexOf(i)];
        card.style.transform = `translate(0,0)`;
        card.style.left = pos[0] + 'px';
        card.style.top = pos[1] + 'px';
    }
}

// Initial positions
renderAllCards();

function animateClockwise() {
    const oldOrder = order.slice();
    // Rotate order: [a,b,c] -> [c,a,b] (clockwise)
    order = [order[2], order[0], order[1]];
    for(let i=0;i<3;i++) {
        const cardEl = document.getElementById(`card${i+1}`);
        const fromIdx = oldOrder.indexOf(i);
        const toIdx = order.indexOf(i);
        const from = positions[fromIdx];
        const to = positions[toIdx];
        let curve;
        if (fromIdx === 0 && toIdx === 1)       curve = curves[0]; // 1->2
        else if (fromIdx === 1 && toIdx === 2)  curve = curves[1]; // 2->3
        else if (fromIdx === 2 && toIdx === 0)  curve = curves[2]; // 3->1
        else if (fromIdx === 1 && toIdx === 0)  curve = [100, 90];
        else if (fromIdx === 2 && toIdx === 1)  curve = [100, 260];
        else if (fromIdx === 0 && toIdx === 2)  curve = [250, 260];
        else                                    curve = [(from[0]+to[0])/2, (from[1]+to[1])/2];
        bezierMove(cardEl, from, to, curve, 1200, () => {
            // After animation, snap to new position and clear transform
            cardEl.style.left = to[0] + 'px';
            cardEl.style.top = to[1] + 'px';
            cardEl.style.transform = 'translate(0,0)';
        });
    }
}

// Click on any card triggers a clockwise round
['card1','card2','card3'].forEach((id) => {
    document.getElementById(id).addEventListener('click', animateClockwise);
});
                "#}
         </script>
     </div>
    }
}
