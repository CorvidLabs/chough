---
change: report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records
artifact: design
---

# Design

## Command

`chough timing [--json] [--cycle current|all] [--stage <stage>] [--case <id>]
[--bug <id>]`: read-only. It writes no file, runs no case and makes no network
request; the only external command is the existing bounded native status read.

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
  failed save writes no gate record. Same fields as today, so no schema bump.
- It records when, never who decided or what was decided.

## Derivation

- Stage segments: walk transitions in file order. Each record ends the open segment
  and, unless it is `pause`, starts the next one in `to`. `pause` to `resume` is a
  paused segment. `iterate` starts a new numbered visit; `enter` starts a side-stage
  segment and `return` resumes the caller. A cycle starts at the first record or at a
  `reset`/`back` whose destination is `intent`. The open segment ends at now and is
  marked ongoing.
- Waiting time: gate-opened to gate-closed pairs by gate text; overlap with each
  stage segment is waiting-on-human; active = segment - paused - waiting (overlapping
  gates are merged so time is never counted twice).
- Tests: observations grouped by case and environment (times, outcomes, count);
  validation runs give started, attempts, failures, elapsed.
- Bugs: created; reproduced = earliest linked observation with outcome failed;
  verified = first note whose text starts with `status: ` and ends the status change
  `-> verified`; fix time = reproduced to verified.
- Native: created and updated from `specsync change status --json`; other gate times
  per the native-source decision; anything else unknown with reason.
- History ranges: per stage (and per bug fix and per gate), across completed cycles,
  count, min, median, max, labelled "estimate from N recorded <items>; not a target".
  Fewer than 3 samples: "too few to estimate".

## Labels and unknowns

Each duration carries a label: `elapsed wall-clock` (never "effort"),
`recorded together` when start and end share a second, `ongoing`, or `unknown` with a
reason (missing timestamp, end before start, gate opened before gate records existed,
native time not exposed, native read unavailable/timed-out, record over its bound).
Unknown values are `null` in JSON and `unknown` in text, never 0. Times print as UTC
ISO 8601 with the raw seconds in JSON.

## Bounds

Transitions read in full up to 16 MiB (beyond: earliest records skipped, report
partial with the skipped byte count); bug, observation and validation records through
`read_record` (64 KiB each) and existing directory loaders; native through the existing
bounded runner. Report output is bounded by the same per-section counts.

## Code layout

New `src/timing.rs` (derivation and rendering, `pub(super)`), wired in `src/main.rs`;
small `pub(super)` accessors in `stage.rs`, `evidence.rs`, `bugs.rs` and
`validate.rs` for the fields it reads; a bounded full log reader in `records.rs`.
`specs/cli/cli.spec.md` files list and command groups gain `timing`.
