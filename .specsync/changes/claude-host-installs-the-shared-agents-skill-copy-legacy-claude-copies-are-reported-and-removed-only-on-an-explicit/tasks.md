---
change: claude-host-installs-the-shared-agents-skill-copy-legacy-claude-copies-are-reported-and-removed-only-on-an-explicit
artifact: tasks
---

# Tasks

- [x] Failure-first regressions and cases TC-cli-claude-host-shares-agents-copy and TC-cli-legacy-claude-copy-removed-only-explicitly
- [x] Remap claude, deduplicate shared directories, report legacy copies, add `skill remove-legacy`
- [x] Update existing host tests only where LIFE-12 changes the expected directory
- [x] Remove this repository's `.claude/` copy with `chough skill remove-legacy --host claude`; update CLAUDE.md, README.md, SKILL.md, docs and examples
