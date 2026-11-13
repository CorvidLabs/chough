---
id: 2026-11-13-fix-249-the-relay-reaches-specsync-s-approval-and-review
title: "Fix #249: the relay reaches SpecSync's approval and review"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-13
time: 2026-11-13T14:18:13Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-13-review-relay.md
  - .specsync/archive/changes/2026-11-13-fix-249-chough-passes-a-relayed-decision-s-words-place-and-recording-agent-to-specsync-s-review-and-approve/
record: .chough/archive/sessions/2026-11-13-review-relay.md
actor:
  - agent: claude-code
    terminal: t382
---
# Fix #249: the relay reaches SpecSync's approval and review

A relayed gate decision that drives a native step passes the human's words, place and recording agent to specsync change approve and change review as --words, --where and --recorded-by, so SpecSync's own records carry the relay; chough change ship's review does too (REQ-records-008, REQ-cli-059).

Verified: the cases red then green, the bug verified, and the full suite 736 of 736. Next: Leif's review and merge.

Read more: [the story](../../.chough/archive/sessions/2026-11-13-review-relay.md) · [the archived change](../../.specsync/archive/changes/2026-11-13-fix-249-chough-passes-a-relayed-decision-s-words-place-and-recording-agent-to-specsync-s-review-and-approve/)
