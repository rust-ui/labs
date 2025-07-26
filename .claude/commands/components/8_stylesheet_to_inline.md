# Convert Small CSS to Inline

1. Find components: `grep -r "Stylesheet" src/__demos__/`
2. Check CSS sizes: `wc -l public/components/*.css | sort -n`
3. For files <50 lines:
  - Remove `use leptos_meta::Stylesheet;`
  - Replace `<Stylesheet href="/file.css" />` with `<style>{r#"CSS_HERE"#}</style>`
  - Copy CSS content into the `r#"..."#` block
  - Delete the unused CSS file: `rm public/components/file.css`