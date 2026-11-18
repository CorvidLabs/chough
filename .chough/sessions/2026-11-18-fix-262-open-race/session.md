---
kind: session
id: 2026-11-18-fix-262-open-race
aliases: [fix-262-open-race]
objective: "No reader can block on a FIFO swapped in between the regular-file check and the open: the type is checked on the opened handle (REQ-history-006, review of #305)"
intent_scope: ""
modules: []
stage: plan
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
observations: []
bugs: []
blockers: []
waiting_on_human:
  - gate: Scope approval of fix-262-open-race
    holds: plan
    drives: approve
    opened: 2026-11-18T01:57:28Z
run_consents: []
next: Wait for Leif's scope decision on fix-262-open-race
---
# No reader can block on a FIFO swapped in between the regular-file check and the open: the type is checked on the opened handle (REQ-history-006, review of #305)

## Goal

No reader can block on a FIFO swapped in between the regular-file check and the open: the type is checked on the opened handle (REQ-history-006, review of #305)

## Where it stands

- open_regular added and used by read_bounded_record and session shelve; the unit test and the special_files and sessions tests pass. The unit test was not seen failing first: the reviewer's replacement between check and open needs an interposer.

## Decisions and why

- Leif, 2026-11-17, answering Fable's question on the agent review of #305: 'Fix first, then merge'. The reader opens without blocking and checks the opened handle; a second check after a blocking open was rejected because that open never returns.
- No requirement text changes (--no-spec-change).

## What was verified

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
