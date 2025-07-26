# Inline CSS into HTML

Move CSS from external files into HTML `<style>` tags for the specified files: $ARGUMENTS

## Process

**Check progress marker `✔️` at top and skip completed steps**

⚠️ *IMPORTANT*: For each steps, you need to commit your work.

For each commit, mark the top of the HTML file this pattern:
- `<!-- INLINE: Step 1 ✔️ -->`
- `<!-- INLINE: Step 1 ✔️ Step 2 ✔️ -->`
- `<!-- INLINE: Step 1 ✔️ Step 2 ✔️ Step 3 ✔️ -->` etc.

⚠️ **IMPORTANT**: If a step has no applicable changes (e.g., CSS file already missing), do NOT create a commit just to update the progress marker. Simply update the marker and proceed directly to the next step with actual changes.

## Steps

### Step 1: Check CSS file eligibility (*COMMIT AFTER COMPLETION*)

1. Locate the adjacent CSS file for the HTML file
   - If no CSS file exists or it's empty, skip to Step 4 (verification only)
2. Count the number of lines in the CSS file
3. **ONLY proceed if CSS file has less than 100 lines and contains content**
4. If CSS file has 100+ lines, skip this process and document the reason
5. Note any JavaScript files that might reference the CSS file

### Step 2: Inline CSS content (*COMMIT AFTER COMPLETION*)

1. Copy the entire CSS content from the external file
2. Create or locate the `<style>` tag in the HTML `<head>` section
3. Paste the CSS content inside the `<style>` tag
4. Ensure proper indentation (2-space indentation for CSS inside `<style>`)
5. Verify that all CSS rules are properly transferred

### Step 3: Remove external CSS file (*COMMIT AFTER COMPLETION*)

1. Remove the `<link rel="stylesheet" href="...">` tag from HTML
2. Delete the external CSS file
3. Ensure no other files reference the deleted CSS file

### Step 4: Verify functionality and naming (*COMMIT AFTER COMPLETION*)

1. Check that all styling is preserved after inlining (if CSS was inlined)
2. Verify that any JavaScript functionality still works
3. Test that the page renders correctly
4. Confirm no broken references or missing styles
5. **If there was only an HTML file**: Update the HTML file name to match the parent folder name (e.g., if parent folder is `accordion`, rename `index.html` to `accordion.html`)

## Important Notes

- **Only inline CSS files with less than 100 lines and with content**
- If no CSS file exists or it's empty, focus on verification and naming
- Keep the CSS content exactly as it was in the external file
- Maintain proper HTML structure and indentation
- Verify that no other files depend on the CSS file being removed
- If the CSS file is shared by multiple HTML files, do NOT inline it
- **Naming convention**: If there's only an HTML file, rename it to match the parent folder name