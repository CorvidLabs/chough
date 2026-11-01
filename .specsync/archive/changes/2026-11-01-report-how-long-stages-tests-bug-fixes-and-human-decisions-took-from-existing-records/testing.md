---
change: report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-020 | `tests/timing.rs`: fixture transitions with fixed timestamps covering advance, iterate, back, reset, skip, pause, resume, enter and return give the expected segments and cycles; paused time is excluded from active time; gate-opened and gate-closed records change no segment boundary or visit count; every reset starts a cycle and back does not; the first record's `from` segment has an unknown start; `done` time is idle and not a sample |
| REQ-cli-020 | gate open and clear through `chough checkpoint` append one record each; reopening an open gate appends nothing; waiting time per stage matches the fixture; a gate open before gate records shows an unknown open time; a gate-opened record whose gate is no longer in `session.json` (edited by hand, or close append failed) shows closed at an unknown time, not ongoing |
| REQ-cli-020 | missing timestamp, end before start and same-second segments report unknown or recorded together, never 0 or negative; fewer than 3 samples report too few to estimate |
| REQ-cli-020 | bugs: created, reproduced and verified times from fixture records; a pruned linked failure uses its ledger time and an unavailable one makes reproduced unknown; verified matches only a `status: <old> -> verified` change; observations grouped per case and environment; a validation run that waited an interval reports its elapsed time labelled total run wall-clock, not test time |
| REQ-cli-020, REQ-cli-003 | native stand-in that is absent, fails or times out gives unknown native times; native records in an unexpected shape give unknown; approval output never contains the fixture's actor or note text; an approval that the stand-in status reports invalid, or that has a later reopening, is shown reopened or not current; more than 10 archived changes give partial with the count skipped |
| REQ-cli-020, REQ-cli-006 | transitions over the bound report partial; every file under `.chough/` and `.specsync/` is byte-identical after `chough timing` |
| REQ-cli-007, REQ-cli-008 | existing checkpoint and stage tests stay green; `chough context` and `next` still parse a log containing gate records |

Fixtures write timestamps directly so results are deterministic; tests that use the
real clock assert only ordering. Regression: full `cargo test`, `fledge lanes run
verify`, and the feature-and-bug exercise on Ubuntu, macOS and Windows CI. Case
definitions for REQ-cli-020 are a post-verification step, added after `change check`
materializes it (not a tasks.md checkbox).
