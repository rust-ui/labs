# Commit Changes

Commit current changes to the local git repository without pushing to remote.

## Usage
Use this command when you want to:
- Commit all staged/unstaged changes with an appropriate message
- Save work locally without pushing to the remote repository
- Follow the project's Git workflow guidelines from CLAUDE.md
- Create incremental commits during development

## Behavior
1. Check git status and recent commits for context
2. Stage relevant files (avoiding sensitive or temporary files)
3. Create a descriptive commit message following conventional commit format
4. Commit changes locally
5. Verify the commit succeeded

## Commit Message Format
Commits must follow conventional commit format:
- `feat: add new component functionality`
- `fix: resolve styling issue in button component`
- `refactor: convert CSS to Tailwind classes`
- `docs: update component documentation`
- `chore: update dependencies`
- `test: add component tests`

## Notes
- Follows Git workflow guidelines in CLAUDE.md
- Uses conventional commit message format (required)
- Includes Claude Code attribution
- Does NOT push to remote (use _commit_push for that)
- Good for frequent local commits during development
