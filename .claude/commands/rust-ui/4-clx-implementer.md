---
name: leptos_ui implementer
descri*ption: Take demo component(s) and use leptos_ui macros (clx! or void!) to create reusable components.
# allowed-tools:
argument-hint: ["demo_file"]
---


## Input

1. *$demo_file*: Demo from `__ready_to_port__/` directory.


## Workflow

1. Look existing patterns like in `card.rs` and how the components are used.
2. **think hard** on only meaningful HTML elements that could benefit that pattern in *$demo_file*.
3. Propose the User a list of component that you can do. Wait for approval.


## Output

Reusable components using `leptos_ui` macros for better mainteanance.
