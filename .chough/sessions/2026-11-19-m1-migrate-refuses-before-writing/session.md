---
kind: session
id: 2026-11-19-m1-migrate-refuses-before-writing
aliases: [m1-migrate-refuses-before-writing]
objective: "Migration refuses before it writes and leaves a working tree: any entry at a target or receipt name counts as taken, parallel applies cannot both apply, a returning 0.5.x record already migrated is refused by name with the generated index rebuilt, and doctor --fix keeps a pointer that names an open session by alias (#376, #370, #337, #378; #375 and #338 await Leif's choice)"
intent_scope: ""
modules: []
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: ""
pr: ""
criteria: [DRIVE-27, LIFE-20, LIFE-21]
requirements: [REQ-records-009, REQ-cli-055]
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Read scoped intent and native status, then choose the next action within the authorized scope.
---
# Migration refuses before it writes and leaves a working tree: any entry at a target or receipt name counts as taken, parallel applies cannot both apply, a returning 0.5.x record already migrated is refused by name with the generated index rebuilt, and doctor --fix keeps a pointer that names an open session by alias (#376, #370, #337, #378; #375 and #338 await Leif's choice)

## Goal

Migration refuses before it writes and leaves a working tree: any entry at a target or receipt name counts as taken, parallel applies cannot both apply, a returning 0.5.x record already migrated is refused by name with the generated index rebuilt, and doctor --fix keeps a pointer that names an open session by alias (#376, #370, #337, #378; #375 and #338 await Leif's choice)

## Where it stands

## Decisions and why

## What was verified

## Open threads

## Timeline

- 2026-11-19T06:11:08Z · start · intent → intent · Migration refuses before it writes and leaves a working tree: any entry at a target or receipt name counts as taken, parallel applies cannot both apply, a returning 0.5.x record already migrated is refused by name with the generated index rebuilt, and doctor --fix keeps a pointer that names an open session by alias (#376, #370, #337, #378; #375 and #338 await Leif's choice) ^t-6a8318
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T06:11:08Z · advance · intent → design ·  ^t-77d10f
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T06:11:09Z · skip · design → plan · Bug fixes to migration refusals and one doctor check; nothing visual or interactive changes ^t-00412e
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-fable-5-2, hc t4b)
