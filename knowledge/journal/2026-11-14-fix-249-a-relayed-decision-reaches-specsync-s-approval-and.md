---
id: 2026-11-14-fix-249-a-relayed-decision-reaches-specsync-s-approval-and
title: "Fix #249: a relayed decision reaches SpecSync's approval and review…"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-14
time: 2026-11-14T20:03:55Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-14-249-relay-to-specsync.md
  - .specsync/archive/changes/2026-11-14-fix-249-a-relayed-decision-reaches-specsync-s-approval-and-review-records-each-value-joined-to-its-flag/
record: .chough/archive/sessions/2026-11-14-249-relay-to-specsync.md
actor:
  - agent: claude-code
---
# Fix #249: a relayed decision reaches SpecSync's approval and review…

Fix #249: a relayed decision reaches SpecSync's approval and review records, each value joined to its flag

Why: 0xLeif decided "Scope approval of fix-249-a-relayed-decision-reaches-specsync-s-approval-and-review-records-each-value-joined-to-its-flag": "approve" (Claude Code desktop chat, 2026-11-14, answering the scope summary for #288); recorded by claude-code; a relayed claim, not authenticated; ran specsync change approve fix-249-a-relayed-decision-reaches-specsync-s-approval-and-review-records-each-value-joined-to-its-flag as 0xLeif

Verified: Ran cargo test --locked --test change -- --exact ship_passes_a_relay_that_starts_with_a_hyphen_as_one_argument_per_field for TC-cli-ship-relay-hyphen-values-join-their-flags: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-14-249-relay-to-specsync.md) · [the archived change](../../.specsync/archive/changes/2026-11-14-fix-249-a-relayed-decision-reaches-specsync-s-approval-and-review-records-each-value-joined-to-its-flag/)
