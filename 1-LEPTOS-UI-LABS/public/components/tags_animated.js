const search = document.querySelector('.search');

const tagsContainer = document.querySelector('.tags');

tagsContainer.addEventListener('click', (e) => {
  const tag = e.target.closest('button');
  if (tag) {
    document.startViewTransition(() => {
      search.appendChild(tag);
    });
  }
});

search.addEventListener('click', (e) => {
  const span = e.target.closest('span');
  if (span) {
    const tag = span.closest('button');
    document.startViewTransition(() => {
      tagsContainer.appendChild(tag);
    });
  }
});