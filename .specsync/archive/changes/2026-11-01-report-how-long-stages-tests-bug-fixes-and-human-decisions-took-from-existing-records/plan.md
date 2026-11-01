---
change: report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records
artifact: plan
---

# Plan

## Approach

Add one read-only report over existing records and one append-only gate record in
the checkpoint command. Keep native state native and label every duration honestly.

## Out of scope

Effort tracking, timers or a running clock; per-user or per-agent attribution;
estimates as targets or deadlines; changing SpecSync or its record format;
reconstructing gate times from Git history; a new record store; network or hosted
reporting; per-case run durations unless the human approves that option below.

## Steps

1. Bounded full transition reader and gate-opened/gate-closed appends in
   `checkpoint_command`, with unit and fixture tests. Size S; uncertainty: clear
   semantics when the same gate text is added twice.
2. Stage segment derivation (cycles, iterations, pause, side stages, waiting
   overlap). Size M; uncertainty: odd but valid histories such as back during a pause.
3. Tests, validation and bug sections. Size S; uncertainty: bug notes edited by hand.
4. Native section per the native-source decision. Size S (status only) to M
   (read native records read-only with shape checks).
5. History ranges and labels, text and JSON rendering. Size S.
6. Docs, skill references, known-previous bundle digests, host copies. Size S.
7. Fixture tests on all three platforms, structural checks. Size M.

Overall relative size: medium; main uncertainty is the native source for gate times.
Single owner (one agent); no parallel file ownership needed.

## Open choices for the human

1. Native gate times: `change status --json` exposes only created and updated.
   Recommendation: read `approvals.json` (gate, timestamp and reopening timestamps
   only; never actor or note), `verification.json`, `review.json` and
   `finalization.json` (timestamp only) read-only, bounded, shape-checked, falling
   back to unknown, with validity taken only from `change status`; history reads the
   10 most recent archived changes, one bounded status read each; and ask SpecSync
   upstream to expose these times in status. Alternative: status only, other native
   times unknown.
2. Per-test run time: case observations hold no duration. Recommendation: add an
   optional `duration_seconds` to case-run observations (additive, absent on old
   records, which stay unknown; touches `src/cases.rs` and the `evidence.rs`
   observation schema). Alternative: leave single-run time unknown, and drop
   `src/cases.rs` from the affected paths.
3. Gate records in `.chough/transitions.jsonl` (recommended: no new store; released
   binaries still parse them) versus a separate `.chough/gates.jsonl`.

## Risks

Misreading wall-clock time as effort (mitigated by labels); idle time in `done`
counted as work (reported separately, excluded from estimates); a stale native
approval shown as current (validity from status only); thin history producing
false confidence (minimum sample rule); SpecSync record shapes changing (shape checks,
unknown); gate records written after `change check` staling verification (written
only by checkpoint calls that already write the session, and the accept-order
guidance from Raven #12 applies unchanged).

## Constraints consulted

specs/cli/context.md lessons (checkbox-free gates, materialize new REQ IDs before
cases, pub(super), disposable Git identity); Raven triage #10 and #12 on Chough writes
and native verification; Invariants 2, 3, 5, 6 and 8; AGENTS.md record ownership.
