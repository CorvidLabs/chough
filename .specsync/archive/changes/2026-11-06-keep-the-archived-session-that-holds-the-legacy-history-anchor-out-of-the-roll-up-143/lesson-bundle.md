# Lesson bundle: keep-the-archived-session-that-holds-the-legacy-history-anchor-out-of-the-roll-up-143

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep the archived session that holds the legacy history anchor out of the roll-up (143)
- **Kind**: BugFix
- **Specs**: history
- **Paths**: hi/knowledge.md, src/sessions.rs, tests/reduce.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: KNOW-29 as amended by Leif on 2026-11-06 (HI captured in hi/knowledge.md) and REQ-history-011: the archived session whose history cursor holds the legacy anchor is never rolled into the vault and does not count toward the five; with seven archived sessions where only the one that would roll first holds the anchor, the move into done keeps it and the five newest others and rolls only the remaining one. The KNOW-29 fixture models a single anchor holder; the new test fails first, no other assertion changes, docs/cli.md says so, and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `252714f3cc740ef6336502d0c13dae091faf9685`
- Base commit: `f35f721ae0049cd6247119b6ff1085108fa4494f`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issue #143, a follow-up to #141. Leif chose option A on 2026-11-06 and amended KNOW-29 in his words ("Amend KNOW-29"), with this wording: "The session that adopted the legacy history (it holds the legacy anchor) is never rolled into the vault; it stays in .chough/sessions/archive in addition to the five, so history never depends on the knowledge directory."

The #141 fix lets `adopted_anchor` find the legacy anchor in sessions rolled into the vault. So today the anchor depends on the knowledge directory: its `library-dir` preference and its files. The KNOW-29 roll-up (REQ-history-011) can roll the session holding the anchor once five newer sessions are archived.

The KNOW-29 test fixture copies the adopting session, cursor included, into each of the seven archived sessions it fabricates, so all eight hold the anchor. In a real project only the adopting session holds it.

## From the change's design.md

# Design

## Approach

In `sessions::roll_up`, each archived session is already read to find its native change. The same read also checks `history_cursor.legacy`, and a session with that field set is left out of the ranking. So it never moves, and the five are counted among the others. Nothing else changes:
- ordering, the no-replace rule and the byte-for-byte move;
- reporting (`applied.session_rollup`);
- `adopted_anchor`, which still finds an anchor already rolled into the vault or shelved (#141).

An archived session that can't be read stays where it is, as today.

## Risks

- **Several archived sessions claiming the anchor:** only possible in hand-made fixtures. All of them would stay; that's safe, and nothing is lost.
- **An anchor holder already rolled into the vault** before this change: it stays there, and #141 still finds it.

## Constraints consulted

KNOW-29 (amended 2026-11-06), REQ-history-011, REQ-history-002, #141.

## From the change's testing.md

# Testing

tests/reduce.rs:
- `seven_archived_sessions` now models a single anchor holder. Each fabricated archived session has `history_cursor.legacy` null; the adopting session, still open, keeps the anchor. The existing KNOW-29 tests keep every assertion.
- `the_roll_up_keeps_the_session_that_holds_the_legacy_anchor` (new) gives s0, the session that would roll first, the adopting session's anchor. At the move into done it expects:
  - `rolled` with only `s6` moved;
  - s0 to s5 still in `.chough/sessions/archive/`;
  - no vault copy of s0.

  It fails first: s0 rolls.

A case `TC-history-roll-up-keeps-the-anchor-holder` binds the new test. `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/history/context.md`
