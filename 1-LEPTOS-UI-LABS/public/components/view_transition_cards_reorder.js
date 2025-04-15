document.querySelector('button').addEventListener('click', (e) => {
	const $cards = Array.from(document.querySelectorAll('.card'));
	shuffle($cards);
	document.startViewTransition(() => {
		$cards.forEach(($card, i) => {
			$card.style.setProperty('order', i);
		});
	});
});
