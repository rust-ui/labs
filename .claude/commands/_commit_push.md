# Commit and Push Changes

Commit current changes and push them to the GitHub repository.

## Usage
Use this command when you want to:
- Commit all staged/unstaged changes with an appropriate message
- Push the commit to the remote GitHub repository
- Follow the project's Git workflow guidelines from CLAUDE.md

## Behavior
1. Check git status and recent commits for context
2. Stage relevant files (avoiding sensitive or temporary files)
3. Create a descriptive commit message following conventional commit format
4. Push changes to origin/master (or current branch)
5. Verify the push succeeded

## Notes
- Follows Git workflow guidelines in CLAUDE.md
- Uses conventional commit message format
- Includes Claude Code attribution
- Only pushes when explicitly requested (this command implies explicit request)