---
change: claude-host-installs-the-shared-agents-skill-copy-legacy-claude-copies-are-reported-and-removed-only-on-an-explicit
artifact: requirements
---

# Requirements

Modify REQ-cli-016: the claude host maps to the shared `.agents/skills` directory, hosts sharing a directory install one copy, an installed `.claude/skills/chough-workflow/` copy is reported as legacy and left unchanged by status, update and init, and an explicit `chough skill remove-legacy` removes only unedited legacy bundle files. Trace adds LIFE-12, LIFE-13 and LIFE-14.
