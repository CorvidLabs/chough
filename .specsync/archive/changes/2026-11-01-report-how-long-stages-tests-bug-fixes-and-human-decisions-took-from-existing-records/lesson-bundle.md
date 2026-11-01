# Lesson bundle — report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Report how long stages, tests, bug fixes and human decisions took, from existing records
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/main.rs, src/timing.rs, src/checkpoint.rs, src/stage.rs, src/records.rs, src/evidence.rs, src/bugs.rs, src/cases.rs, src/validate.rs, src/skill.rs, tests/, docs/, README.md, hi/, INTENT.md, skills/chough-workflow/, .claude/skills/chough-workflow/, .agents/skills/chough-workflow/
- **Acceptance**: `chough timing` (text, and `--json`) reports each stage visit as a segment with start, end and elapsed wall-clock time derived from `.chough/transitions.jsonl`: each advance, iterate, back, reset, skip, pause, resume, side-stage enter and return record ends the open segment and starts the next, while `gate-opened` and `gate-closed` records neither start nor end a segment; iterations and side-stage visits are counted separately, and paused time is shown on its own and excluded from active time. Every `reset`, whatever its destination, starts a new cycle and `back` never does; the first cycle starts at the first record, and the segment before it is shown with an unknown start because init and migrate write no transition. Time in `done` until the next reset is reported as idle between cycles and is excluded from stage time and from estimates (LIFE-10, REQ-cli-020).
- **Acceptance**: `chough checkpoint --awaiting-human "<gate>"` appends one `gate-opened` transition (kind, current stage, gate text, time, revision) only when that gate is not already open, and `--clear-awaiting-human` appends one `gate-closed` transition per gate it clears, in the same command that already writes the checkpoint. The report shows each gate's open, close and waiting time, gates still open as open with elapsed-so-far, and per stage active time separately from waiting-on-human time. Gate records are reconciled with the checkpoint's current awaiting-human list: a gate opened in transitions, never closed there and no longer in the list is reported closed at an unknown time, and a gate in the list without a `gate-opened` record, such as one opened before this change, has an unknown open time. The record says only when a gate was opened and cleared; it never records or implies who decided or what was decided (LIFE-10, REQ-cli-020, REQ-cli-007, REQ-cli-008).
- **Acceptance**: Tests and bug fixes: per case and environment the report lists observation times and outcomes and, for validation runs, their recorded attempts and total run wall-clock time (`elapsed_seconds`, which includes interval waits and resumes and is labelled so, not as per-test time); per bug it shows created, reproduced (the earliest linked failed observation, using the retention ledger's recorded time for a pruned one, and unknown with reason unavailable when any linked observation is neither present nor in the ledger), verified (the first note whose recorded changes include `status: <old> -> verified`), and the elapsed time between them. Durations no record holds, such as a single case run's run time today, are reported unknown, never zero (LIFE-10, REQ-cli-020).
- **Acceptance**: Native lifecycle times are taken only from native SpecSync output or native records read read-only (change created and updated from `specsync change status --json`; definition approval, verification, review and finalization as resolved by the open native-source question). From native approval records only the gate, timestamp and reopening timestamps are read, never the actor, note or any other field; approval validity comes only from `change status`, and an approval that status reports invalid, or that a later reopening superseded, is reported as reopened or not current, never as a current approval. The current change takes one bounded status read; history across archived changes takes one bounded read per archived change for at most the 10 most recent, and older ones are reported partial with the count skipped. A time native output does not expose, an unavailable, failed, oversized or timed-out native read, or an unrecognised record shape is shown unknown with its reason; Chough copies no native state into its own records (LIFE-10, LIFE-6, REQ-cli-003, REQ-cli-020).
- **Acceptance**: Honest labels: every duration is labelled elapsed wall-clock time, not effort; history summaries show the sample count with minimum, median and maximum, labelled an estimate from recorded history and not a target, and fewer than 3 samples are reported as too few to estimate; a missing or unparseable timestamp, or an end earlier than its start, is shown unknown, never zero or negative (LIFE-10, LIFE-7, REQ-cli-020).
- **Acceptance**: Bounded and read-only: transitions are read up to a fixed bound and other records through the existing record bounds, anything beyond a bound is reported partial with what was skipped, and running the report leaves every file under `.chough/` and `.specsync/` byte-identical, runs no case and makes no network request (REQ-cli-006, REQ-cli-020).
- **Acceptance**: docs/cli.md and docs/architecture.md describe the command, the two new transition kinds and the labels; the canonical skill (checkpoint and plan references) tells agents to estimate from `chough timing` ranges and to say when history is too thin, host copies are reinstalled through the known-previous bundle mechanism, and isolated fixtures with fixed timestamps cover every transition kind, gate pairs, missing and out-of-order times, bounds and byte-identical records; the full suite and native structural checks pass on Ubuntu, macOS and Windows CI.

## Evidence

- Verification commit: `b5038b45f0d0d1cbd33b9e060bcda929e3b0bc80`
- Base commit: `85650d93ad9a811c2c54e08256620d351754da8d`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The human (0xLeif) agreed criterion LIFE-10 in this wording, captured with `hi`:
"I can see how long each stage, test, bug fix and human decision actually took, so I
can estimate future work from real history." The plan reference already asks agents
to estimate increments by relative size without inventing hours; today nothing
shows them what earlier work actually took.

Most of the needed times already exist, spread over records:

- `.chough/transitions.jsonl`: `at_unix_seconds` on every advance, iterate, back,
  reset, pause, resume, skip and side-stage enter or return.
- `.chough/evidence/*.json`: `observed_at_unix_seconds` per observation, with
  case, environment and outcome; `.chough/validation/` run summaries carry
  `started_at_unix_seconds` and `elapsed_seconds`.
- `.chough/bugs/*.json`: `created_at_unix_seconds`, `updated_at_unix_seconds`, and
  notes with `at_unix_seconds` and Chough-written `status: a -> b` text.
- SpecSync: `change status --json` exposes `change.created_at` and
  `change.updated_at`; approval, verification, review and finalization timestamps
  live in the change's native record files (see research).

What is missing is when a human gate opened and closed. `awaiting_human` in
`.chough/session.json` is a list that `--clear-awaiting-human` empties, so the time
a gate waited is lost. That is the one new record this change adds.

Constraints a resuming session needs: draft only until the human approves the scope;
`.chough/` is deliberately not an affected path (Chough writes after `change check`
stale native verification, Raven issues #10 and #12); Chough's own history is thin
(18 transitions on main, several written in the same second when backfilled), so the
report must say when history is too thin rather than invent a range.

## From the change's design.md

# Design

## Command

`chough timing [--json] [--cycle current|all] [--stage <stage>] [--case <id>]
[--bug <id>]`: read-only. It writes no file, runs no case and makes no network
request. Its only external commands are bounded `specsync change status <id> --json`
reads through the existing runner: one for `session.native_change`, and, for history,
one per archived change for at most the 10 most recent archived changes (by archive
directory date); older archived changes are skipped and the report says partial with
the count skipped. Native record files are read only under the native-source choice.

## The one new record

`checkpoint_command` (src/checkpoint.rs) appends, in the same call that already
writes `.chough/session.json`:

```text
{"schema_version":1,"at_unix_seconds":T,"kind":"gate-opened","from":"<stage>","to":"<stage>","reason":"<gate text>","gate":null,"revision":"<HEAD>","refresh":[]}
{"schema_version":1,"at_unix_seconds":T,"kind":"gate-closed","from":"<stage>","to":"<stage>","reason":"<gate text>","gate":null,"revision":"<HEAD>","refresh":[]}
```

- `gate-opened` only for a gate text not already in `awaiting_human`;
  `gate-closed` once per gate removed by `--clear-awaiting-human`. Clears run before
  additions (existing order), so clear-and-reopen in one call writes closed then opened.
- The session write happens first; the transition is appended after it succeeds, so a
  failed save writes no gate record. A failed append after a successful save, or a gate
  removed by editing `session.json` by hand, leaves an unmatched record; the report
  reconciles that (see Waiting time). Same fields as today, so no schema bump.
- It records when, never who decided or what was decided.

## Derivation

- Stage segments: walk transitions in file order. Stage kinds (advance, iterate, back,
  reset, skip, pause, resume, enter, return) end the open segment and start the next
  one in `to`; `pause` to `resume` is a paused segment. `gate-opened` and
  `gate-closed` neither start nor end a segment and never add a visit; they only define
  waiting intervals. `iterate` starts a new numbered visit; `enter` starts a side-stage
  segment and `return` resumes the caller. The open segment ends at now and is marked
  ongoing.
- First segment: init and migrate write no transition (only `stage.rs` calls
  `log_transition`), so the segment in the first record's `from` stage has an unknown
  start (reason `no transition before the first record`); its time is unknown, never 0.
- Cycles: every `reset`, whatever its destination, starts a new cycle; `back` never
  does (it is rework inside the cycle, counted as such). The first cycle starts at the
  first record. A cycle can start at any stage, so stages it never visited are absent,
  not zero. On Chough's own log (first record build -> test, resets accept -> intent,
  done -> plan, done -> build) this gives 4 cycles, one per objective.
- Idle: a segment in `done` (entered by advance or release, left by the next reset) is
  reported as idle between cycles. It is not stage time, not cycle active time and not
  an estimate sample.
- Waiting time: gate-opened to gate-closed pairs by gate text, reconciled with the
  current `session.awaiting_human`: opened and still listed means open (ongoing);
  opened, never closed in transitions and no longer listed means closed at an unknown
  time (reason `cleared without a gate record`); listed without a gate-opened record
  means an unknown open time (reason `opened before gate records or record missing`).
  Overlap of a known interval with each stage segment is waiting-on-human; active =
  segment - paused - waiting (overlapping gates are merged so time is never counted
  twice). A gate with an unknown end gives an unknown waiting time and leaves the
  affected segments' active time unknown, not inflated.
- Tests: observations grouped by case and environment (times, outcomes, count).
  Validation runs give started, attempts, failures and `elapsed_seconds`, labelled
  `total run wall-clock, including intervals and resumes`: `validate.rs` accumulates it
  across interval waits and resumes and keeps no per-attempt time, so it is never shown
  as test duration.
- Bugs: created; reproduced = earliest linked failed observation. Present observations
  give their own time; a pruned one gives the `observed_at_unix_seconds` and outcome
  its retention-ledger entry recorded, labelled pruned; if any linked observation is
  neither present nor in the ledger (unavailable, or the ledger is unreadable),
  reproduced is unknown with reason `unavailable`, because the missing one could be
  the earlier failure. Verified = the first note whose changes prefix (the `; `-joined
  `field: old -> new` entries before the first `. `, as `bug update` writes them)
  includes `status: <old> -> verified`; fix time = reproduced to verified. A free-text
  note written to look like that prefix cannot be told apart, so the verified time is
  labelled `from note text`; with no matching note it is unknown.
- Native: created and updated from `specsync change status --json`. Under the
  native-source choice, from `approvals.json` only `approvals[].gate`,
  `approvals[].timestamp` and reopening timestamps are read; `actor`, `note` and every
  other field are never read into the report. Validity comes only from status
  (`summary.approval_valid` and related fields), never from the file: an approval that
  status reports invalid, or that a later reopening supersedes, is shown as `reopened`
  or `not current` with its historical time, never as a current approval. Verification,
  review and finalization take only `timestamp`. Anything else is unknown with reason.
- History ranges: per stage (and per bug fix and per gate), across completed cycles,
  excluding idle `done` time and segments with an unknown start or end,
  count, min, median, max, labelled "estimate from N recorded <items>; not a target".
  Fewer than 3 samples: "too few to estimate".

## Labels and unknowns

Each duration carries a label: `elapsed wall-clock` (never "effort"),
`recorded together` when start and end share a second, `ongoing`, or `unknown` with a
reason (missing timestamp, end before start, no transition before the first record,
gate opened before gate records existed, gate cleared without a gate record, linked
observation unavailable, native time not exposed, native read unavailable/timed-out,
record over its bound).
Unknown values are `null` in JSON and `unknown` in text, never 0. Times print as UTC
ISO 8601 with the raw seconds in JSON.

## Bounds

Transitions read in full up to 16 MiB (beyond: earliest records skipped, report
partial with the skipped byte count); bug, observation and validation records through
`read_record` (64 KiB each) and existing directory loaders; the retention ledger
through its existing 16 MiB bound; native through the existing bounded runner, at most
1 + 10 status reads (beyond 10 archived changes: partial, count skipped) and native
record files through `read_record`. Report output is bounded by the same per-section counts.

## Code layout

New `src/timing.rs` (derivation and rendering, `pub(super)`), wired in `src/main.rs`;
small `pub(super)` accessors in `stage.rs`, `evidence.rs` (including the retention
ledger's observed time and outcome), `bugs.rs` and `validate.rs` for the fields it
reads; a bounded full log reader in `records.rs`. `src/cases.rs` and a `duration_seconds`
field in the `evidence.rs` observation schema change only if open choice 2 is approved;
if it is declined, `src/cases.rs` is dropped from the affected paths before approval.
`specs/cli/cli.spec.md` files list and command groups gain `timing`.

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
