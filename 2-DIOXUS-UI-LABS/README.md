# Run the project

## 1. Run Tailwind

```bash
pnpm install

npx @tailwindcss/cli -i ./style/input.css -o ./style/output.css --watch
```

## 2. Run Dioxus

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

