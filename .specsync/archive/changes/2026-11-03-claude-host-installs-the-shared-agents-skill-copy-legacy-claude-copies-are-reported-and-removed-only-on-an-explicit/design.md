---
change: claude-host-installs-the-shared-agents-skill-copy-legacy-claude-copies-are-reported-and-removed-only-on-an-explicit
artifact: design
---

# Design

`HOSTS` maps `claude` to `.agents/skills`. Host lists are deduplicated by directory before planning or reporting, so `--host claude --host openai` plans one bundle; reports keep the first requested host name and list every host that shares the directory. A new `LEGACY` table names `(claude, .claude/skills)`. `skill status` and `skill update` add a `legacy` array: each installed legacy directory with every file's state (current, known-outdated, edited, unknown) and the explicit removal command. Neither ever writes there.

`chough skill remove-legacy --host claude` (no host means every installed legacy directory) refuses with nothing removed unless the shared `.agents/skills/chough-workflow/SKILL.md` exists. It removes only bundle files whose bytes are the current bundle's or a known earlier bundle's (KNOWN_PREVIOUS), keeps edited and unknown files, then removes directories left empty, and reports removed and kept files.
