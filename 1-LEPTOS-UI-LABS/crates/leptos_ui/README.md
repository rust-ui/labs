# Leptos UI

A utility crate for creating UI components in Leptos with Tailwind CSS class merging support. Built on of [tw_merge](https://crates.io/crates/tw_merge).

## Features

- `clx!` macro for creating components with merged Tailwind classes

## Usage

### Basic Component with `clx!`

```rust
// components/ui/card.rs
use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Card, div, "rounded-lg p-4", "bg-sky-500"} // 🩵
}

pub use components::*;

// components/demos/demo_card.rs
#[component]
pub fn DemoCard() -> impl IntoView {
    view! {
        <Card>"Card bg-sky-500 🩵"</Card>
        <Card class="bg-orange-500">"Card bg-orange-500 🧡"</Card>
    }
}
```


## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
leptos_ui = "0.1"
```

## License

MIT
