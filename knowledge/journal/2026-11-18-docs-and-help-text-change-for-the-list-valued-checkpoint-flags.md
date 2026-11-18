---
id: 2026-11-18-docs-and-help-text-change-for-the-list-valued-checkpoint-flags
title: Docs and help-text change for the list-valued checkpoint flags and…
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T03:37:40Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-17-fix-301.md
  - .specsync/archive/changes/2026-11-18-fix-301/
record: .chough/archive/sessions/2026-11-17-fix-301.md
actor:
  - agent: claude-code
    terminal: t28
---
# Docs and help-text change for the list-valued checkpoint flags and…

Docs and help-text change for the list-valued checkpoint flags and docs/cli.md (issue 301)

Why: 0xLeif decided "Scope approval of fix-301": "approved" (Claude Code chat (hc t28), 2026-11-17, in reply to the #301 scope summary (docs/cli.md rows, help text for all 11 checkpoint flags, help --help exits 0, changelog fragment)); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve fix-301 as 0xLeif

Verified: fledge run fmt, lint and test pass (785 passed, 0 failed), including the two new tests in tests/cli.rs: chough help --help exits 0 with the top-level help, and chough checkpoint --help prints a description under each of the eleven list-valued flags (fails on the old checkpoint.rs); docs/cli.md has no change ready text, per Leif's decision

Read more: [the story](../../.chough/archive/sessions/2026-11-17-fix-301.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-301/)
