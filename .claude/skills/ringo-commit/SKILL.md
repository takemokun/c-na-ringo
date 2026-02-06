---
name: ringo-commit
description: Create git commits with emoji-based semantic commit messages using ringo conventions.
argument-hint: [feat|fix|chore|refactor|docs]
---

# Ringo Commit

Create git commits using emoji-based semantic commit conventions, then push and create a PR to main — all in one flow.

## Emoji Commit Type Mapping

| Type | Emoji | Meaning | Example |
|------|-------|---------|---------|
| feat | 🍎 | New feature | `🍎 add ringo-new-skill` |
| fix | 🍏 | Bug fix | `🍏 resolve quiz scoring bug` |
| chore | 🎵 | Maintenance | `🎵 update dependencies` |
| refactor | ✨ | Code improvement | `✨ simplify SRS algorithm` |
| docs | 📝 | Documentation | `📝 update README` |

## Commit Message Format

```
<emoji> <description>
```

- The emoji replaces the conventional `type:` prefix
- Description: lowercase, imperative mood, no trailing period
- No scope prefix needed — keep it simple
- **NO Co-Authored-By, no Claude attribution** — commit messages are the user's own

## Workflow

All steps (commit → push → PR) run as a single continuous flow. Do NOT ask for confirmation between steps.

### 1. Analyze Changes

Run in parallel:
- `git status` — see staged and unstaged files
- `git diff --cached` — review staged changes
- `git diff` — review unstaged changes
- `git log --oneline -5` — check recent commit style

### 2. Stage Files

- If no files are staged, identify relevant changed files and stage them by name
- Never use `git add -A` or `git add .` — always stage specific files
- Warn if any file looks like it contains secrets (`.env`, credentials, keys)

### 3. Detect Commit Type

Auto-detect from changes, or use the user-provided type argument:

| Signal | Type |
|--------|------|
| New files added | feat 🍎 |
| Bug/error fixes in existing code | fix 🍏 |
| Config, deps, tooling changes | chore 🎵 |
| Restructured existing code, no behavior change | refactor ✨ |
| Markdown, comments, README changes only | docs 📝 |

If ambiguous, ask the user which type fits best.

### 4. Generate Commit Message

Compose a message following the format:

```
<emoji> <short imperative description>
```

- Description should be 3-10 words, summarizing the "why" not the "what"

### 5. Execute Commit

```bash
git commit -m "<emoji> <description>"
```

### 6. Verify Commit

Run `git log --oneline -1` to confirm the commit was created with the correct emoji format.

### 7. Push

Immediately after commit (no confirmation needed):

- Push to the current remote tracking branch with `git push`
- If no upstream is set, use `git push -u origin <branch-name>`
- Never force push unless the user explicitly requests it

### 8. Create PR to main

Immediately after push (no confirmation needed):

- Use `gh pr create` with the emoji commit message as the PR title
- Base branch is always `main`
- PR body format:

```bash
gh pr create --base main --title "<emoji> <description>" --body "$(cat <<'EOF'
## Summary
- <1-3 bullet points describing changes>

## Test plan
- [ ] <testing checklist items>
EOF
)"
```

- Do NOT include "Generated with Claude Code" or any Claude attribution in the PR body
- If a PR already exists for this branch, skip PR creation and print the existing PR URL instead
- Return the PR URL to the user when done

## Examples

```
🍎 add ringo-commit skill
🍏 fix SRS interval calculation
🎵 update node dependencies
✨ simplify quiz difficulty selection
📝 update learning guidelines in README
```

## Attribution Policy

僕の手柄にしたいから、これらを守って

- **NO `Co-Authored-By` lines** in commit messages
- **NO "Generated with Claude Code"** in PR bodies
- **NO Claude/AI attribution** anywhere — all commits and PRs are the user's own work

## Notes

- This is a git operation skill, not a language learning skill
- Always show the proposed commit message to the user before committing
- If pre-commit hooks fail, fix the issue and create a NEW commit (never amend)
- The full flow (commit → push → PR) runs without pausing for user confirmation
