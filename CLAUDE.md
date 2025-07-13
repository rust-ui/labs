# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Rust UI Labs** is a collaborative workspace for developing UI components for Rust web frameworks (Leptos and Dioxus). This is a temporary development repository where components are prototyped before integration into [Rust UI](https://rust-ui.com/).

## Development Guidelines

### Primary Development
```bash
cargo leptos watch  # Main development server (127.0.0.1:3000)
```

### Performances Guidelines

For maximum efficiency, whenever you need to perform multiple independent operations, invoke all relevant tools simultaneously rather than sequentially.


## Architecture

### Project Structure
- **`src/`** - All Rust source code (unified single-crate structure)
- **`public/`** - Static assets (CSS, JS, images)
- **`style/`** - Tailwind CSS configuration

### Key Technologies
- **Leptos 0.8** - Main web framework with SSR
- **Tailwind CSS 4.1.3** - Utility-first CSS framework
- **leptos_ui** - UI component library with `clx!` macro

### Component Organization
- **Demo-driven development** - Each component has its own demo in `src/__demos__/`
- **Shared components** - Reusable components in `src/shared/components/`


## Development Guidelines

### Git Workflow
See `.claude/commands/github` directory.


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

