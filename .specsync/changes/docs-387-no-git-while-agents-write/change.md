---
id: docs-387-no-git-while-agents-write
state: approved
type: documentation
base_commit: 9d260310bca8553a0203e5ef90bc8372b4e15aae
---

# Docs 387 no git while agents write

## Intent

docs 387 no git while agents write

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The workflow skill's checkpoint reference, in the bundle and in an installed copy, and `docs/cli.md` each say not to run `git pull`, `git merge`, `git rebase`, `git checkout` or `git stash` in a worktree while another agent works there, why, and that a lone agent may run one between its own commands.

## No-spec Rationale

Not applicable
