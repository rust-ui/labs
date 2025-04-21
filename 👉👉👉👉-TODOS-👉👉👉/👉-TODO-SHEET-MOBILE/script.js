// add light dismiss
demo.addEventListener('click', ({target:dialog}) => {
  if (dialog.nodeName === 'DIALOG')
    dialog.close('dismiss')
})