---
change: keep-the-archived-session-that-holds-the-legacy-history-anchor-out-of-the-roll-up-143
artifact: testing
---

# Testing

tests/reduce.rs:
- `seven_archived_sessions` now models a single anchor holder. Each fabricated archived session has `history_cursor.legacy` null; the adopting session, still open, keeps the anchor. The existing KNOW-29 tests keep every assertion.
- `the_roll_up_keeps_the_session_that_holds_the_legacy_anchor` (new) gives s0, the session that would roll first, the adopting session's anchor. At the move into done it expects:
  - `rolled` with only `s6` moved;
  - s0 to s5 still in `.chough/sessions/archive/`;
  - no vault copy of s0.

  It fails first: s0 rolls.

A case `TC-history-roll-up-keeps-the-anchor-holder` binds the new test. `fledge lanes run verify` passes on Linux.
