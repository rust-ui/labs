# clx! Macro Usage


## Instructions

- Replace `<div>` tags that accept children with `clx!` macro using meaningful names.
- Keep self-closing tags as regular HTML:
  - Empty divs: `<div class="..."></div>`
  - Images: `<img>` 
- Keep a line break before the view macro.


Example:
```rust
clx! {Gallery, div, "flex justify-around"}
<Gallery><img src="..." /></Gallery>
```


