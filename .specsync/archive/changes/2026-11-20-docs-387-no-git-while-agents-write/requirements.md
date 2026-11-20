---
change: docs-387-no-git-while-agents-write
artifact: requirements
---

# Requirements

REQ-cli-040 gains one sentence and one criterion; nothing else changes: the writer
lock orders Chough's writers only, so the skill and the docs tell agents not to run
`git pull`, `merge`, `rebase`, `checkout`, `stash` or a like command in a worktree
while another agent works there; a lone agent may, between its own commands (#387).
