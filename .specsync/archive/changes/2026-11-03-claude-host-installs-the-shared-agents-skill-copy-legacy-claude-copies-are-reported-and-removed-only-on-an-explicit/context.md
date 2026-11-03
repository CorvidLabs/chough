---
change: claude-host-installs-the-shared-agents-skill-copy-legacy-claude-copies-are-reported-and-removed-only-on-an-explicit
artifact: context
---

# Context

Leif (0xLeif) answered a multiple-choice interview on 2026-11-03: "Which AI tools should get a copy of the instructions? Only the shared .agents copy", and at 8:27 AM MT, choosing among a pointer skill, removal and keeping both copies, "Remove .claude entirely". He chose removal knowing that docs/integrations.md records Claude Code 2.1.316 discovering `.claude/skills` and not `.agents/skills` alone (answer: NONE). HI LIFE-12, LIFE-13 and LIFE-14 capture this.

At main 0363d4a `chough init --host claude` writes a full bundle to `.claude/skills/chough-workflow/`, and this repository commits byte-identical full copies under both `.agents/` and `.claude/`.

Human authority: Leif (0xLeif) in chat 2026-11-03 8:09-8:27 AM MT, interview answers "Only the shared .agents copy" and "Remove .claude entirely", relayed by the parent agent as his intent approval; his next decision is final acceptance ("Ask me only at intent and final acceptance"). Covered scope: the claude host mapping, legacy-copy reporting and an explicit removal command in src/skill.rs and src/main.rs, their tests, REQ-cli-016, removal of this repository's own `.claude/` copy, and docs. Limits: other hosts keep their directories; no other project's files are touched; no release or deployment; edited legacy files are never removed.
