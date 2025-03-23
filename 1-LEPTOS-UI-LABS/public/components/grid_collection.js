const grid = document.querySelector('.grid');

grid.addEventListener('click', async (e) => {
  const path = e.composedPath();
  const item = path.find(el => el.classList?.contains('grid-item'));
  const closeIcon = path.find(el => el.classList?.contains('close-icon'));

  if (closeIcon) {
    item.classList.add('active');

    const transition = document.startViewTransition({
      update: () => {
        item.classList.remove('expanded');
      },
      types: ['collapse']
    });

    await transition.finished;
    item.classList.remove('active');
  }
  else {
    if(!item.classList.contains('expanded')) {
      item.classList.add('active');

      const transition = await document.startViewTransition({
        update: () => {
          item.classList.add('expanded');
        },
        types: ['expand']
      });

      await transition.finished;
      item.classList.remove('active');
    }
  }
});