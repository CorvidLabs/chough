---
kind: session
id: 2026-11-18-fix-263
aliases: [fix-263]
objective: "migrate --apply refuses before writing when a planned target that differs only in case from a removed 0.5.x file already exists with other bytes (REQ-records-009, #263)"
intent_scope: ""
modules: []
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: fix-263
pr: ""
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
cases: [TC-records-migrate-refuses-a-separate-case-colliding-target]
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human:
  - gate: Scope approval of fix-263
    holds: plan
    drives: approve
    opened: 2026-11-18T15:03:43Z
run_consents: []
next: Wait for Leif's scope approval of fix-263
---
# migrate --apply refuses before writing when a planned target that differs only in case from a removed 0.5.x file already exists with other bytes (REQ-records-009, #263)

## Goal

migrate --apply refuses before writing when a planned target that differs only in case from a removed 0.5.x file already exists with other bytes (REQ-records-009, #263)

## Where it stands

## Decisions and why

- The pre-write check exempts a case-colliding target only when it is the removed file under its other spelling: the other spelling still opens and the folder does not list both names. The session's criteria list DRIVE-27 by my mistake; LIFE-20 is the one that applies

## What was verified

## Open threads

## Timeline

- 2026-11-18T14:24:30Z · start · intent → intent · migrate --apply refuses before writing when a planned target that differs only in case from a removed 0.5.x file already exists with other bytes (REQ-records-009, #263) ^t-fef20b
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T14:24:30Z · advance · intent → design ·  ^t-7f060e
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T14:24:30Z · skip · design → plan · Migration refusal change; nothing visual or interactive changes ^t-5bd574
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T15:03:43Z · gate-opened · plan → plan · Scope approval of fix-263 ^t-1bede6
  - native_change: fix-263
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
