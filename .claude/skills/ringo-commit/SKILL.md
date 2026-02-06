---
name: ringo-commit
description: Create git commits with emoji-based semantic commit messages using ringo conventions.
argument-hint: [feat|fix|chore|refactor|docs]
---

# Ringo Commit

Create git commits using emoji-based semantic commit conventions.

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

## Workflow

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

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
```

- Description should be 3-10 words, summarizing the "why" not the "what"
- Use a HEREDOC for the commit command to preserve formatting

### 5. Execute Commit

```bash
git commit -m "$(cat <<'EOF'
<emoji> <description>

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
EOF
)"
```

### 6. Verify

Run `git log --oneline -1` to confirm the commit was created with the correct emoji format.

### 7. Push (optional)

After commit, ask the user if they want to push:

- Push to the current remote tracking branch with `git push`
- If no upstream is set, use `git push -u origin <branch-name>`
- Never force push unless the user explicitly requests it

### 8. Create PR (optional)

After push, ask the user if they want to create a PR:

- Use `gh pr create` with the emoji commit message as the PR title
- PR body format:

```bash
gh pr create --title "<emoji> <description>" --body "$(cat <<'EOF'
## Summary
- <1-3 bullet points describing changes>

## Test plan
- [ ] <testing checklist items>
EOF
)"
```

- Do NOT include "Generated with Claude Code" in the PR body (僕の手柄にしたいから。)
- Base branch defaults to `main` unless the user specifies otherwise
- Return the PR URL to the user when done

## Examples

```
🍎 add ringo-commit skill
🍏 fix SRS interval calculation
🎵 update node dependencies
✨ simplify quiz difficulty selection
📝 update learning guidelines in README
```

## Notes

- This is a git operation skill, not a language learning skill
- Always show the proposed commit message to the user before committing
- If pre-commit hooks fail, fix the issue and create a NEW commit (never amend)
