# clx! Macro Usage


## Instructions

- Replace `<div>` tags that accept children with `clx!` macro using meaningful names.
- Keep self-closing tags as regular HTML:
  - Empty divs: `<div class="..."></div>`
  - Images: `<img>` 
- Keep a line break before the view macro.
- Add constants for repeated values like URLs, strings, etc.


Example:
```rust
const IMAGE_1: &str = "https://example.com/image1.jpg";

clx! {Gallery, div, "flex justify-around"}
<Gallery><img src={IMAGE_1} /></Gallery>
```


