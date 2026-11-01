---
id: report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records
state: draft
type: feature
base_commit: 85650d93ad9a811c2c54e08256620d351754da8d
---

# Report how long stages, tests, bug fixes and human decisions took, from existing records

## Intent

Report how long stages, tests, bug fixes and human decisions took, from existing records

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- `chough timing` (text, and `--json`) reports each stage visit as a segment with start, end and elapsed wall-clock time derived from `.chough/transitions.jsonl`: every advance, iterate, back, reset, skip, pause, resume, side-stage enter and return starts or ends a segment, iterations and side-stage visits are counted separately, and paused time is shown on its own and excluded from active time. History is grouped into cycles that each begin at init or at a reset or back to intent (LIFE-10, REQ-cli-020).
- `chough checkpoint --awaiting-human "<gate>"` appends one `gate-opened` transition (kind, current stage, gate text, time, revision) only when that gate is not already open, and `--clear-awaiting-human` appends one `gate-closed` transition per gate it clears, in the same command that already writes the checkpoint. The report shows each gate's open, close and waiting time, gates still open as open with elapsed-so-far, and per stage active time separately from waiting-on-human time. The record says only when a gate was opened and cleared; it never records or implies who decided or what was decided, and gates opened before this change are reported with unknown open times (LIFE-10, REQ-cli-020, REQ-cli-007, REQ-cli-008).
- Tests and bug fixes: per case and environment the report lists observation times and outcomes and, for repeated validation runs, their recorded attempts and elapsed seconds; per bug it shows created, first linked failed observation (reproduced), the status note that moved it to verified, and the elapsed time between them. Durations no record holds, such as a single case run's run time today, are reported unknown, never zero (LIFE-10, REQ-cli-020).
- Native lifecycle times are taken only from native SpecSync output or native records read read-only (change created and updated from `specsync change status --json`; definition approval, verification, review and finalization as resolved by the open native-source question). A time native output does not expose, an unavailable, failed, oversized or timed-out native read, or an unrecognised record shape is shown unknown with its reason; Chough copies no native state into its own records (LIFE-10, LIFE-6, REQ-cli-003, REQ-cli-020).
- Honest labels: every duration is labelled elapsed wall-clock time, not effort; history summaries show the sample count with minimum, median and maximum, labelled an estimate from recorded history and not a target, and fewer than 3 samples are reported as too few to estimate; a missing or unparseable timestamp, or an end earlier than its start, is shown unknown, never zero or negative (LIFE-10, LIFE-7, REQ-cli-020).
- Bounded and read-only: transitions are read up to a fixed bound and other records through the existing record bounds, anything beyond a bound is reported partial with what was skipped, and running the report leaves every file under `.chough/` and `.specsync/` byte-identical, runs no case and makes no network request (REQ-cli-006, REQ-cli-020).
- docs/cli.md and docs/architecture.md describe the command, the two new transition kinds and the labels; the canonical skill (checkpoint and plan references) tells agents to estimate from `chough timing` ranges and to say when history is too thin, host copies are reinstalled through the known-previous bundle mechanism, and isolated fixtures with fixed timestamps cover every transition kind, gate pairs, missing and out-of-order times, bounds and byte-identical records; the full suite and native structural checks pass on Ubuntu, macOS and Windows CI.

## No-spec Rationale

Not applicable
