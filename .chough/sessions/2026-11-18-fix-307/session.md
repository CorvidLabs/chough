---
kind: session
id: 2026-11-18-fix-307
aliases: [fix-307]
objective: "chough change ready names an affected path whose owning module the change does not declare, so the approval does not refuse on it after ready said nothing is missing (REQ-cli-067, #307)"
intent_scope: ""
modules: []
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: fix-307
pr: ""
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human:
  - gate: Scope approval of fix-307
    holds: plan
    drives: approve
    opened: 2026-11-18T14:43:15Z
run_consents: []
next: Wait for Leif's scope approval of fix-307; then implement the finding in src/ready.rs
---
# chough change ready names an affected path whose owning module the change does not declare, so the approval does not refuse on it after ready said nothing is missing (REQ-cli-067, #307)

## Goal

chough change ready names an affected path whose owning module the change does not declare, so the approval does not refuse on it after ready said nothing is missing (REQ-cli-067, #307)

## Where it stands

## Decisions and why

- A missing owning module is a new kind of finding, so REQ-cli-067 is modified by a delta; the finding names the module once with its paths and says declaring it can add questions, which the next report lists

## What was verified

## Open threads

## Timeline

- 2026-11-18T14:41:50Z · start · intent → intent · chough change ready names an affected path whose owning module the change does not declare, so the approval does not refuse on it after ready said nothing is missing (REQ-cli-067, #307) ^t-4f58ae
  - revision: ea159cfef2259a81d1855371d351d543a0d833a4
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T14:41:50Z · advance · intent → design ·  ^t-7c97f0
  - revision: ea159cfef2259a81d1855371d351d543a0d833a4
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T14:41:50Z · skip · design → plan · Report text change in a CLI command; nothing visual or interactive changes ^t-e2f689
  - revision: ea159cfef2259a81d1855371d351d543a0d833a4
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T14:43:15Z · gate-opened · plan → plan · Scope approval of fix-307 ^t-2d33ff
  - native_change: fix-307
  - revision: ea159cfef2259a81d1855371d351d543a0d833a4
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
