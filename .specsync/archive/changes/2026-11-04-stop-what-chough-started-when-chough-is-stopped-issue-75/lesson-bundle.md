# Lesson bundle: stop-what-chough-started-when-chough-is-stopped-issue-75

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Stop what Chough started when Chough is stopped (issue 75)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/records.rs, tests/stop.rs, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: On Unix SIGINT or SIGTERM or SIGHUP sent to chough during chough cases run stops the case command and its descendants and chough ends by that signal,The same signals during a native SpecSync read by chough next stop the native command and its descendants,An interrupted run records no observation and no artifact and leaves the checkpoint and the .chough directory byte-identical,A signal that chough inherited as ignored stays ignored and the deadline and held-output behavior of REQ-cli-011 is unchanged,docs/cli.md states the behavior and each new test fails on main edbc3c1 and fledge lanes run verify passes

## Evidence

- Verification commit: `688ccc380e6391319e730908e802fb39a5ff1a8f`
- Base commit: `5b69b2f0ffee9c0a8f322e557421d8f4de8a3e07`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Kimi's field lab round 3 (2026-11-03, Chough 4daf861) is an agent lab review, not a
human pass. Its finding N2, reproduced by corvid-agent on v0.2.0 (edbc3c1), is
https://github.com/CorvidLabs/chough/issues/75: when Chough is stopped during
`chough cases run` or a native SpecSync read, the command it started keeps running.
`run_bounded` in src/records.rs starts each command in its own process group so the
deadline can stop the whole tree, but nothing stops that group when Chough itself is
signalled, and the separate group also keeps a terminal Ctrl-C from reaching it.

Leif confirmed the criterion at 7:07 PM MT on 2026-11-03 (card t819s9328): "Yes, when I
stop Chough everything it started stops too and nothing partial is recorded", answering
"When you stop Chough (Ctrl-C or a normal kill), everything it started stops too, and
nothing half-done gets saved." It is captured as FLOW-14. A SIGKILL watchdog was offered
and not chosen, so a hard kill of Chough is out of scope.

Authority: Leif's 2026-11-03 7:49 AM MT standing Chough maintainer authority for bounded
fixes to Chough bug tickets, his 6:52 PM MT answer "File all three as Chough issues and
fix the clear bugs as small PRs", and the FLOW-14 confirmation above.

Out of scope: SIGKILL, Windows console events (behavior unchanged), and Chough's own
short `git` reads, which stay in Chough's process group and so already receive a
terminal Ctrl-C.

## From the change's design.md

# Design

- src/records.rs: a small Unix-only `interrupt` module. While `run_bounded` owns a child,
  it installs a handler for SIGINT, SIGTERM and SIGHUP that only stores the signal number
  in an atomic (async-signal-safe). A signal whose inherited disposition is ignored is
  left ignored. The previous dispositions are restored when the run ends.
- The runner's poll loop (every 25 ms) checks the stored signal. When one arrived, it
  stops the child's process group with the existing `stop_tree`, reaps the child and
  returns. `run_bounded` then restores the dispositions and, when a signal arrived at any
  point during the run, ends Chough by that same signal (default disposition, then
  `raise`), so callers never record an observation, artifact or checkpoint change, and
  the parent sees the usual death by signal.
- The watch is a guard: dropping the last one restores the dispositions and, when a
  signal arrived, ends Chough by it; during a panic it only restores them. Nested
  watches share one count, and a signal blocked or ignored when the first watch starts
  keeps that disposition.
- `cases run` holds a watch from before the binding runs until its observation is
  saved. It takes the Git snapshot first, then calls `stop_if_noted`, the last point at
  which a stop records nothing, and only then writes the artifact and the observation,
  which run no command; a stop during those writes lets both finish and then ends
  Chough. Chough's own Git commands (`evidence` snapshot, `hash-object`, ancestry and
  HEAD reads) run through `run_owned`: their own process group, stopped with Chough.
  So no cleanup of partial files is needed. Gates, schemas, output keys, the
  deadline and held-output handling are unchanged, and no dependency is added (`libc` is
  already a Unix dependency).

## From the change's testing.md

# Testing

- `tests/stop.rs` `stopping_a_case_run_stops_its_command_and_records_nothing`: for each of
  SIGINT, SIGTERM and SIGHUP, a `command` case starts `sh` with a background `sleep`;
  once it is running, the signal is sent to `chough cases run`. Chough ends by that
  signal, no process of the case survives, and the `.chough` directory (session,
  evidence, artifacts) is byte-identical to before. Then, with SIGHUP ignored by the
  parent (`trap '' HUP`), SIGHUP leaves the case running and the run records its
  observation as before, so an inherited ignored signal stays ignored.
- `tests/stop.rs` `stopping_a_native_read_stops_specsync_and_records_nothing`: the same
  for a stub `specsync` started by `chough next --json`.
- `tests/stop.rs` `stopping_a_case_run_while_it_saves_stops_git_and_records_nothing`
  (review on #79): the bound command exits at once, then every `git` call stalls in a
  stand-in that runs the real Git afterwards. For each of SIGINT, SIGTERM and SIGHUP
  sent during that stall, Chough ends by the signal, the stalled Git is gone and
  `.chough` is byte-identical. At e97633e it fails twice over: the Git stand-in
  survives, and a `.chough/artifacts/OBS-*.log` is left with no observation.
- `tests/stop.rs` `a_stop_signal_inherited_as_blocked_stays_blocked`: with SIGTERM
  blocked when Chough starts, a SIGTERM leaves the case running and the run records
  its observation as before.
- Each original test fails with `src/` at main edbc3c1, because the command survives.
- Existing deadline and held-output tests in tests/cases.rs pass unchanged.
- `cargo test --locked` and `fledge lanes run verify` pass.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-011 | `tests/stop.rs`: `stopping_a_case_run_stops_its_command_and_records_nothing`, `stopping_a_native_read_stops_specsync_and_records_nothing`, `stopping_a_case_run_while_it_saves_stops_git_and_records_nothing`, `a_stop_signal_inherited_as_blocked_stays_blocked`. |

## Where these lessons go

- `specs/cli/context.md`
