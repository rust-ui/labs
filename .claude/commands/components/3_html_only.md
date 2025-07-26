# Inline CSS into HTML

Move CSS from external files into HTML `<style>` tags for the specified files: $ARGUMENTS


## Process

### 1. Move CSS inside HTML

Run **subagents** for every folders that is relevant.

1. Check every folder that contains `✔️`
2. Inside it, check if there is a css file. If not, exit this script.
3. **IMPORTANT: Check if there is a JS file (script.js). If there is a JS file, SKIP this folder entirely.**
4. If there CSS, check if **less than 80 lines**.
5. If so, move the CSS directly inside the HTML.

### 2. Have only HTML file

1. When ### 1. is done, then take the **EXACT NAME** of the parent folder and replace the html file name with it.
2. Then, move the html file inside `✔️ 0. ONLY_HTML` directory.
3. Then delete the previous now unused folder.