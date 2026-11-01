---
change: report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records
artifact: design
---

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
