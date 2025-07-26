# HTML-Only Reactive Pattern

Check only for files that have JS:
- grep `.js` files
- grep `script` inside `.html` files.


## Pattern
Replace JavaScript with inline `onclick` attributes for simple state toggles.

## Implementation
- **Open**: `onclick="this.classList.add('active')"`
- **Close**: `onclick="document.querySelector('.target').classList.remove('active')"`
- **Toggle**: `onclick="this.classList.toggle('active')"`
- **... And other pattern like this**


## When to Use
Simple UI state changes that don't need complex logic or data binding.