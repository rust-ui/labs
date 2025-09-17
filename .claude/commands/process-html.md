---
name: Process HTML
description: Process HTML using sequential agents Workflow.
argument-hint: [html_file]
# allowed-tools: TODO
---


# Process HTML using sequential agents Workflow.

## Input

- `$HTML_FILE`: Path to the HTML file to be processed. **REQUIRED**
- `$PORT_NUMBER`: In not provided, default to `8080`. If used, try `8081` etc.



## Workflow (sequential)

1. Launch Live server with `open http://localhost:$PORT_NUMBER/$HTML_FILE`. `
2. Use the command `1-html-css-cleaner` and execute the instruction using the variables.
3. Use the command `2-tailwind-converter` and execute the instruction using the variables.



## Output

- Cleaned HTML file with Tailwind CSS and proper styling conventions.

