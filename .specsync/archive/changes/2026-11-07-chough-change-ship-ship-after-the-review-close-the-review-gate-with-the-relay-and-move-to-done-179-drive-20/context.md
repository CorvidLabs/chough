---
change: chough-change-ship-ship-after-the-review-close-the-review-gate-with-the-relay-and-move-to-done-179-drive-20
artifact: context
---

# Context

Issue #179, slice 2 of "Chough drives SpecSync" (DRIVE-20: "Chough runs SpecSync's agent steps itself: change new, the interview answers, check --commit, ship or finalize, and status"). Slice 1 (#171, merged in #174) added `chough change new`, `answer`, `check` and `status`. Leif approved the slice order on 2026-11-06, relayed by the orc: slice 2 is "`chough change ship`: ship or finalize after the review, then the done move". During #174's re-approval he chose option A, which moves the walkthrough's direct calls in this slice.

## Decisions that bind this change

- **The relay goes with the ship.** The skill's closeout says nothing writes to `.chough/` between `change check --commit` and finalize, because SpecSync's verification digest covers `.chough/`. So the review gate can only be closed after ship, and a one-command closeout must take the relay with it. The relay is checked in full before ship, so a bad value never leaves a shipped change with its gate open.
- **`ship` covers finalize.** `specsync change ship` preflights and finalizes, so there is no separate `chough change finalize`.
- **Review stays direct.** `specsync change review` is the native record of Leif's review. Writing it from Chough is slice 5 (FLOW-19), after spec-sync #900.
- **IDs.** REQ-cli-059 is this change's; REQ-cli-058 is #177's, agreed with t27.
- **Ruled out:**
  - A second native read: closing the gate reads no native state, so the move into done keeps its one read.
  - Rewriting `stage advance` or `checkpoint`: the steps after ship run Chough's own commands.
  - `library index` inside ship: the agent still runs it before committing.
