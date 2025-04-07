const widget = document.querySelector('.widget')
const toggle = document.querySelector('.toggle')

toggle.addEventListener('click', () => {
  widget.classList.toggle('active')
})