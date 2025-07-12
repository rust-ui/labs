# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Rust UI Labs** is a collaborative workspace for developing UI components for Rust web frameworks (Leptos and Dioxus). This is a temporary development repository where components are prototyped before integration into [Rust UI](https://rust-ui.com/).

## Development Commands

### Primary Development
```bash
cargo leptos watch              # Main development server (127.0.0.1:3000)
```

### Code Quality
```bash
# Rust formatting and linting
cargo fmt                      # Format Rust code
cargo clippy                   # Rust linting
cargo clippy --fix            # Auto-fix clippy issues

# Spell checking
typos                          # Check for typos in source code
typos -w                       # Auto-fix typos

# JavaScript/CSS formatting and linting  
npm run format                 # Format JS/CSS with Biome
npm run check                  # Check JS/CSS with Biome
npm run check:fix             # Auto-fix Biome issues
```

### Maintenance
```bash
rustup update                  # Update Rust toolchain
cargo install-update -a       # Update installed cargo tools
cargo update                   # Update dependencies
git pull origin master --rebase
```

## Architecture

### Project Structure
- **`src/`** - All Rust source code (unified single-crate structure)
- **`public/`** - Static assets (CSS, JS, images)
- **`style/`** - Tailwind CSS configuration

### Key Technologies
- **Leptos 0.8** - Main web framework with SSR
- **Axum 0.8** - Web server framework
- **Tailwind CSS 4.1.3** - Utility-first CSS framework
- **leptos_ui** - UI component library with `clx!` macro
- **leptos-struct-table** - Data table components

### Component Organization
- **Demo-driven development** - Each component has its own demo in `src/__demos__/`
- **Shared components** - Reusable components in `src/shared/components/`
- **Query-based routing** - URL parameters select which demo to display
- **Component registration** - All demos registered in `src/all_demos.rs`

## Development Guidelines

### CSS Conversion Process
1. **Use `this__pattern` naming** for custom CSS classes that cannot be converted to Tailwind
2. **Prefer Tailwind over custom CSS** when possible
3. **CSS-to-Tailwind workflow** using Cursor rules:
   - Step 1: Add Tailwind comments to CSS classes (`.cursor/rules/css-1-fill-tailwind-comments.mdc`)
   - Step 2: Apply Tailwind classes to `clx!` components (`.cursor/rules/css-2-fill-clx-components.mdc`)

### JavaScript Philosophy
- **Keep existing JavaScript** - Focus on CSS conversion, not JS replacement
- **Prefer inline JS** over separate script files for simple interactions
- **Avoid wasm-bindgen/web_sys** unless absolutely necessary

### Code Patterns
- **Use `clx!` macro** for styled components: `clx! {ComponentName, div, "tailwind classes"}`
- **Use `div!` macro** for unstyled components: `div! {ComponentName, "css-class"}`
- **Import pattern**: Components re-exported from modules via `pub use components::*;`

## File Structure Patterns

### Demo Structure
```
src/__demos__/
├── demo_name.rs               # Main demo implementation
├── demo_name/                 # Supporting files (if needed)
│   ├── components.rs          # Demo-specific components
│   └── style.css             # Demo-specific styles
```

### Component Structure  
```rust
use leptos::prelude::*;
use leptos_ui::{clx, div};

mod components {
    use super::*;
    clx! {StyledComponent, div, "tailwind classes"}
    div! {UnstyledComponent, "css-class"}
}

pub use components::*;
```

## Testing and Quality

- **No dedicated test commands** - Manual testing via demo interface
- **Code quality tools**: `cargo clippy`, `cargo fmt`, `npm run check`
- **Live reload** - Leptos watch provides hot reloading during development

## Important Notes

- **Nightly Rust required** - Project uses Rust nightly features
- **VSCode extensions**: Rust Analyzer, Biome JS
- **Temporary repository** - Components eventually move to main Rust UI project
- **Discord community**: https://discord.gg/mbszS27TqA