# Dioxus UI

A utility crate for creating UI components in Dioxus with Tailwind CSS class merging support. Built on [tw_merge](https://crates.io/crates/tw_merge).

## Features

- `clx!` macro for creating components with merged Tailwind classes

## Usage

### Basic Component with `clx!`

```rust
// components/ui/card.rs
use dioxus::prelude::*;
use dioxus_ui::clx;

mod components {
    use super::*;
    clx! {Card, div, "rounded-lg p-4", "bg-sky-500"} // 🩵
}

pub use components::*;

// components/demos/demo_card.rs
#[component]
pub fn DemoCard() -> Element {
    rsx! {
        Card { "Card bg-sky-500 🩵" }
        Card { class: "bg-orange-500", "Card bg-orange-500 🧡" }
    }
}
```


## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dioxus_ui = "0.1"
```

## License

MIT
