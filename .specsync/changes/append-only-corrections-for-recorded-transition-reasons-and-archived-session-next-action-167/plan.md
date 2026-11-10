---
change: append-only-corrections-for-recorded-transition-reasons-and-archived-session-next-action-167
artifact: plan
---

# Plan

## Approach

A new `correction` event kind (from == to == current stage) carries `{target, statement}`, validated like the relayed-by and direct-write payloads; `chough correct` appends it after checking the target exists and is not itself a correction. next/status read corrections of the active session's current cycle. src/timing.rs (module reporting) treats the kind as known, starting or ending no segment.

## Out of scope

Rewriting or annotating archived session files in place, corrections of non-event records (decisions, open questions), and any hi rewording.

## Steps

1. Draft the REQ-history-015 delta (done).
2. Implement the kind, command and readers with a RED-first regression (done).
3. Verify with change check --commit, refresh knowledge, open the PR for acceptance.

## Risks

Older builds refuse a history holding a correction event ("unknown transition kind"); the requirement names this and the change log will name the first release that reads it, matching the REQ-history-013/014 pattern.

## Constraints consulted

specs/history/context.md lessons; REQ-history-002 immutability kept: corrections are new events.
