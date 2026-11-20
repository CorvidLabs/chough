---
id: 2026-11-20-change-ready-counts-only-a-change-s-own-work-after-a-rebase
title: change ready counts only a change's own work after a rebase
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-20
time: 2026-11-20T14:17:24Z
kind: shipped
links:
  - 419
  - .chough/archive/sessions/2026-11-20-406-rebase-own-work.md
  - .specsync/archive/changes/2026-11-20-fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main/
record: .chough/archive/sessions/2026-11-20-406-rebase-own-work.md
actor:
  - agent: t4c
    model: claude-opus-5-5
    terminal: t4c
---
# change ready counts only a change's own work after a rebase

chough change ready no longer names the default branch's files as outside the declared paths after a rebase onto it or a squash merge of it: a committed file whose content at HEAD is the default branch's, at its tip or at the merge base, is not the change's own, never on the default branch itself; a missing base commit is said plainly (#406).

Verified at df9914cd: eleven verify steps and four cases on macOS; the tester's PASS (rebased branch 690 and 684 findings down to 9 and 3, every dropped one main's; 66 changes, none dropped); tester2's Linux PASS; corvid-agent's approval; fable-lead's read; main 643420a9 merged in at 0b50b7aa, own lines unchanged, lane green.

Read more: the pull request 419 · [the story](../../.chough/archive/sessions/2026-11-20-406-rebase-own-work.md) · [the archived change](../../.specsync/archive/changes/2026-11-20-fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main/)
