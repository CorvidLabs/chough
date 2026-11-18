---
id: 2026-11-18-next-status-and-doctor-warn-during-a-git-merge-or-rebase
title: next, status and doctor warn during a git merge or rebase
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T19:24:41Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-266.md
  - .specsync/archive/changes/2026-11-18-fix-266/
record: .chough/archive/sessions/2026-11-18-fix-266.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# next, status and doctor warn during a git merge or rebase

next, status and doctor warn during a git merge or rebase, and writes refuse while .chough paths are unmerged (REQ-cli-069, #266)

Why: The rule is a new REQ-cli-069, with the doctor finding defined there beside REQ-history-012's, since #250 and #320 also change REQ-history-012. Writes are refused at the writer lock and before an evidence entry is written, so a refused writer leaves nothing; the merge drivers take no lock and still run.

Verified: Ran cargo test --locked --test git_merge -- --exact a_merge_conflict_in_chough_warns_and_refuses_writes for TC-cli-git-merge-in-progress-warns-and-refuses-writes: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-266.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-266/)
