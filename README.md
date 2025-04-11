# General Purpose

This repository is designed to be a collaborative working space for creating components for Leptos and Dioxus.

It's only a temporary repository to test and develop components. Once they are ready, they are integrated directly into [Rust UI](https://rust-ui.com/).


# Installation

Please refer to [Leptos](https://github.com/leptos-rs/leptos?tab=readme-ov-file#nightly-note/) or [Dioxus](https://github.com/DioxusLabs/dioxus?tab=readme-ov-file#running-the-examples) documentation.


Don't forget to :
- `rustup update`
- `cargo install-update -a`
- `cargo update`
  
- `git pull`



# 👉 TODOS

You'll find the todos in the **Issues** tab. Thanks Florian for the idea :)



# CSS

If possible, try to use `this__pattern` for custom CSS classes that you didn't manage to convert to Tailwind.
This way, we can distinguish them easily.

Example:

```css
.img__holder {
    clip-path: polygon(37.5% 20%, 62.5% 20%, 62.5% 80%, 37.5% 80%);
    transform: rotate(30deg);
}
```



# JS

At the moment, it's probably better to **keep the JS as it is** and not use Rust for interacting with the DOM.
While using wasm-bindgen or web_sys is possible, it seems overkill for the moment.

The main goal here is to convert index.html into reusable components in Rust.

If possible, you can try to use **inline JS** to avoid having a `script.js` file to manage.

Example:

Instead of having this:

```html
<button id="my__button">Show an alert</button>

<script>
  document.getElementById('my__button').addEventListener('click', () => {
    alert('Hello from JS!');
  });
</script>
```

Prefer to have this:

```html
<button onclick="alert('Hello from JS!');">
  "Show an alert"
</button>
```

PS: This is not mandatory, just a nice to have 😄



# 👾 Discord

I'll soon create a Discord server to discuss about the components and the project, and Rust over all (job opportunities, etc).

When it's ready I'll share the link here.




# Contact

If you have any questions, please contact on Linkedin [@max-wells](https://www.linkedin.com/in/maxime-montfort/).



