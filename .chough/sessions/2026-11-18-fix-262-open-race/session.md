---
kind: session
id: 2026-11-18-fix-262-open-race
aliases: [fix-262-open-race]
objective: "No reader can block on a FIFO swapped in between the regular-file check and the open: the type is checked on the opened handle (REQ-history-006, review of #305)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-262-open-race
pr: ""
criteria: [DRIVE-14]
requirements: [REQ-history-006]
cases: [TC-history-a-fifo-at-the-open-is-refused-without-blocking]
design: []
files: [src/records.rs, src/sessions.rs]
observations: [2026-11-18-tc-history-a-fifo-at-the-open-is-refused-without-blocking]
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-262-open-race
    holds: accept
    drives: review
    opened: 2026-11-18T02:11:26Z
run_consents: []
next: Wait for Leif's implementation review of fix-262-open-race; read the PR's reviews first; do not ship
---
# No reader can block on a FIFO swapped in between the regular-file check and the open: the type is checked on the opened handle (REQ-history-006, review of #305)

## Goal

No reader can block on a FIFO swapped in between the regular-file check and the open: the type is checked on the opened handle (REQ-history-006, review of #305)

## Where it stands

- open_regular added and used by read_bounded_record and session shelve; the unit test and the special_files and sessions tests pass. The unit test was not seen failing first: the reviewer's replacement between check and open needs an interposer.

## Decisions and why

- Leif, 2026-11-17, answering Fable's question on the agent review of #305: 'Fix first, then merge'. The reader opens without blocking and checks the opened handle; a second check after a blocking open was rejected because that open never returns.
- No requirement text changes (--no-spec-change).
- 0xLeif decided "Scope approval of fix-262-open-race": "approve all three, record as 0xLeif" (Claude Code session with Fable, 2026-11-17, in reply to Fable's report of the gates on #308, #305 and #306); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-262-open-race as 0xLeif

## What was verified

- [Ran cargo test --locked --bin chough -- --exact records::tests::a_fifo_at_the_moment_of_the_open_is_refused_without_blocking for TC-history-a-fifo-at-the-open-is-refused-without-blocking: passed (exit 0).](evidence/2026-11-18-tc-history-a-fifo-at-the-open-is-refused-without-blocking.md)

## Open threads

## Timeline

- 2026-11-18T01:55:16Z · start · intent → intent · No reader can block on a FIFO swapped in between the regular-file check and the open: the type is checked on the opened handle (REQ-history-006, review of #305) ^t-07040f
  - revision: caa6ee65a46ec3f4798e0a64c22bb263d062814a
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T01:55:16Z · advance · intent → design ·  ^t-33a93d
  - revision: caa6ee65a46ec3f4798e0a64c22bb263d062814a
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T01:55:16Z · skip · design → plan · Record reading change; nothing visual or interactive changes ^t-e2691c
  - revision: caa6ee65a46ec3f4798e0a64c22bb263d062814a
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T01:57:28Z · gate-opened · plan → plan · Scope approval of fix-262-open-race ^t-e1d6eb
  - native_change: fix-262-open-race
  - revision: caa6ee65a46ec3f4798e0a64c22bb263d062814a
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T02:05:51Z · gate-closed · plan → plan · Scope approval of fix-262-open-race ^t-91959b
  - native_change: fix-262-open-race
  - revision: a58e0be0ed9dd3f03515697eb3a7a934f86c9b01
  - relayed_by:
    - decided_by: 0xLeif
    - words: approve all three, record as 0xLeif
    - where: Claude Code session with Fable, 2026-11-17, in reply to Fable's report of the gates on #308, #305 and #306
    - recorded_by: fable-t4b
    - native: specsync change approve fix-262-open-race
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T02:11:25Z · advance · plan → build · native:approved ^t-c1b5f0
  - native_change: fix-262-open-race
  - gate: native:approved
  - revision: a58e0be0ed9dd3f03515697eb3a7a934f86c9b01
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T02:11:25Z · advance · build → test · native:approved ^t-b3ec95
  - native_change: fix-262-open-race
  - gate: native:approved
  - revision: a58e0be0ed9dd3f03515697eb3a7a934f86c9b01
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T02:11:25Z · advance · test → accept · native:approved ^t-70f753
  - native_change: fix-262-open-race
  - gate: native:approved
  - revision: a58e0be0ed9dd3f03515697eb3a7a934f86c9b01
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T02:11:26Z · gate-opened · accept → accept · Implementation review of fix-262-open-race ^t-306f2d
  - native_change: fix-262-open-race
  - revision: a58e0be0ed9dd3f03515697eb3a7a934f86c9b01
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
