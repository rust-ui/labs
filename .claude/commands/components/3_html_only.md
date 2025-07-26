# Inline CSS into HTML

Move CSS from external files into HTML `<style>` tags for the specified files: $ARGUMENTS


## Process

### 1. Move CSS inside HTML

Run **subagents** for every folders that is relevant.

1. Check every folder that contains `✔️`
2. Inside it, check if there is a css file. If not, exit this script.
3. If there CSS, check if **less than 80 lines**.
4. If so, move the CSS drectly inside the HTML.

### 2. Have only HTML file

1. When ### 1. is done, then take the **EXACT NAME** of the parent folder and replace the html file name with it.
2. Then, move the html file inside `✔️ 0. ONLY_HTML` directory.
3. Then delete the preveious now unused folder.