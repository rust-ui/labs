const tabs = document.querySelectorAll('.tab');

tabs.forEach(tab => {
  tab.addEventListener('click', () => {
    document.startViewTransition(() => {
      document.querySelector('.tab.active').classList.remove('active');
      tab.classList.add('active');
    });
  });
});