---
kind: session
id: 2026-11-06-make-the-timing-trace-test-fail-only-when-tracing-changes-the
aliases:
  - make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
objective: "chough#169: the timing trace test fails only when tracing changes the report (DRIVE-19)"
intent_scope: DRIVE-19
modules: [reporting]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
pr: ""
criteria: []
requirements: []
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Await scope approval through the orc; then replace the retry loop
closed: 2026-11-06
outcome: done
receipts: []
---
# chough#169: the timing trace test fails only when tracing changes the report (DRIVE-19)

## Goal

chough#169: the timing trace test fails only when tracing changes the report (DRIVE-19)

## Where it stands

## Decisions and why

- DRIVE-19 captured from Leif via the orc ('Capture both')
- No runtime clock switch (issue #30): the fix is an exact clock-shift comparison in the test

## What was verified

## Open threads

## Timeline

- 2026-11-06T20:32:02Z · advance · intent → design ·  ^t-245971
  - native_change: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
  - revision: 167790489735b3e6a88ec21a20bb2530d4c96138
- 2026-11-06T20:37:03Z · advance · design → plan ·  ^t-629293
  - native_change: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
  - revision: c6f4e24d73a18bc9f7a7703eb0bb184ba4a865e3
- 2026-11-06T20:37:04Z · advance · plan → build · native:approved ^t-d50a26
  - native_change: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
  - gate: native:approved
  - revision: c6f4e24d73a18bc9f7a7703eb0bb184ba4a865e3
- 2026-11-06T20:37:04Z · advance · build → test · native:approved ^t-b0cccb
  - native_change: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
  - gate: native:approved
  - revision: 15c5f7e0579d3fe02e69758dce37f325eecb64c7
- 2026-11-06T20:37:33Z · advance · test → accept · native:approved ^t-c62866
  - native_change: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
  - gate: native:approved
  - revision: ec52e49132c293a423ad6ef3f2aa6039daa42d2b
- 2026-11-06T21:17:34Z · advance · accept → done · native:archived ^t-1d7308
  - native_change: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
  - gate: native:archived
  - revision: 30e3f2d9351fdb6024d47dccd5a881fcaeb9c803


Closed 2026-11-06: done.
