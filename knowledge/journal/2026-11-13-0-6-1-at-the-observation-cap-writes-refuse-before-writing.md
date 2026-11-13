---
id: 2026-11-13-0-6-1-at-the-observation-cap-writes-refuse-before-writing
title: "0.6.1: at the observation cap, writes refuse before writing"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-13
time: 2026-11-13T03:08:31Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-13-observation-cap.md
  - .specsync/archive/changes/2026-11-13-0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough/
record: .chough/archive/sessions/2026-11-13-observation-cap.md
actor:
  - agent: claude-code
    terminal: t382
---
# 0.6.1: at the observation cap, writes refuse before writing

At REQ-cli-032's 400-observation cap, every write that adds an observation refuses before running or writing anything, naming chough session shelve <id>; next and the leave-test gate read a case's latest run only from the observations the checkpoint references.

Verified: the case red then green, the bug verified, and the full suite 731 of 731. Next: Leif's review and merge.

Read more: [the story](../../.chough/archive/sessions/2026-11-13-observation-cap.md) · [the archived change](../../.specsync/archive/changes/2026-11-13-0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough/)
