// getComputedStyle($0).getPropertyValue('view-transition-name');

const shuffle = (array) => {
	for (let i = array.length - 1; i >= 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[array[i], array[j]] = [array[j], array[i]];
	}
}

document.querySelector('button').addEventListener('click', (e) => {
	const $cards = Array.from(document.querySelectorAll('.card'));
	shuffle($cards);
	document.startViewTransition(() => {
		$cards.forEach(($card, i) => {
			$card.style.setProperty('order', i);
		});
	});
});