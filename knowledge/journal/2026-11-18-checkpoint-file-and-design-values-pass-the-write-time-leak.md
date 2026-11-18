---
id: 2026-11-18-checkpoint-file-and-design-values-pass-the-write-time-leak
title: checkpoint --file and --design values pass the write-time leak guard
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T16:22:51Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-268.md
  - .specsync/archive/changes/2026-11-18-fix-268/
record: .chough/archive/sessions/2026-11-18-fix-268.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# checkpoint --file and --design values pass the write-time leak guard

checkpoint --file and --design values pass the write-time leak guard (REQ-records-006, #268)

Why: No requirement text changes: REQ-records-006 already requires every value Chough writes to pass the leak check, so this is --no-spec-change. The guard adds the checkpoint's files and design references; the other references are IDs that validation already limits to an ID form.

Verified: Ran cargo test --locked --test leaks -- --exact evidence_add_in_a_session_holding_a_leak_writes_nothing for TC-records-evidence-add-in-a-leaking-session-writes-nothing: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-268.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-268/)
