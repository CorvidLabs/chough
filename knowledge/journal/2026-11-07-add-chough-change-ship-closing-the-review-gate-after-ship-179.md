---
id: 2026-11-07-add-chough-change-ship-closing-the-review-gate-after-ship-179
title: Add chough change ship, closing the review gate after ship (#179)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-07
time: 2026-11-07T00:12:23Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-chough-change-ship-ship-after-the-review-close-the-review-gate.md
  - .specsync/archive/changes/2026-11-07-chough-change-ship-ship-after-the-review-close-the-review-gate-with-the-relay-and-move-to-done-179-drive-20/
record: .chough/archive/sessions/2026-11-06-chough-change-ship-ship-after-the-review-close-the-review-gate.md
actor:
  - agent: claude-code
    terminal: t382
---
# Add chough change ship, closing the review gate after ship (#179)

Slice 2 of DRIVE-20 (#179, REQ-cli-059) added chough change ship: after the implementation review it ships the native change, closes the review gate with the relayed review, and moves the story to done. The relay goes with ship because nothing may write Chough's records between check and finalize, so the gate closes after ship and the relay is checked in full beforehand.

Closing the gate before ship, a separate finalize command, a library index inside ship, and rewriting stage advance or checkpoint were all rejected; the follow-up steps run Chough's own commands.

The maintainer approved slice 2 and passed the implementation review on PR #182, both relayed rather than authenticated. The native change was archived and the story closed done on 2026-11-07.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-chough-change-ship-ship-after-the-review-close-the-review-gate.md) · [the archived change](../../.specsync/archive/changes/2026-11-07-chough-change-ship-ship-after-the-review-close-the-review-gate-with-the-relay-and-move-to-done-179-drive-20/)
