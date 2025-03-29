const search = document.querySelector('.search');

const tagsContainer = document.querySelector('.tags');

search.addEventListener('click', (e) => {
  const span = e.target.closest('span');
  if (span) {
    const tag = span.closest('button');
    document.startViewTransition(() => {
      tagsContainer.appendChild(tag);
    });
  }
});