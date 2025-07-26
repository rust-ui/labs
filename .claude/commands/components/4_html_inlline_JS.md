# HTML-Only Reactive Pattern

## Pattern
Replace JavaScript with inline `onclick` attributes for simple state toggles.

## Implementation
- **Open**: `onclick="this.classList.add('active')"`
- **Close**: `onclick="document.querySelector('.target').classList.remove('active')"`
- **Toggle**: `onclick="this.classList.toggle('active')"`

## Requirements
- CSS handles all animations via `.active` class
- No separate JavaScript files needed
- Works for simple show/hide, menu toggles, modals

## When to Use
Simple UI state changes that don't need complex logic or data binding.