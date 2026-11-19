---
change: docs-387-no-git-while-agents-write
artifact: requirements
---

# Requirements

REQ-cli-040 gains one sentence and one criterion; nothing else changes: the writer
lock orders Chough's writers only, so the skill and the docs tell agents not to run
`git merge`, `rebase`, `checkout` or `stash` in a worktree while Chough writers run
there (#387).
