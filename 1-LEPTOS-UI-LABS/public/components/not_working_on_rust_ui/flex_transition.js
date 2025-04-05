const button = document.querySelector('button');
const container = document.querySelector('.container-flex-transition');

button.addEventListener('click', () => {
  document.startViewTransition(() => {
    container.classList.toggle('toggle');
  })
});