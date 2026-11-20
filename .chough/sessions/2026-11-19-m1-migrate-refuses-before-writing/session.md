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
native_change: fix-m1-migrate-refuses-before-writing
pr: ""
criteria: [DRIVE-27, LIFE-20, LIFE-21]
requirements: [REQ-records-009, REQ-cli-055, REQ-records-015]
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Gate \"Scope approval of fix-m1-migrate-refuses-before-writing (native approval)\" closed (approved); run `chough stage advance` to move on."
---
# Migration refuses before it writes and leaves a working tree: any entry at a target or receipt name counts as taken, parallel applies cannot both apply, a returning 0.5.x record already migrated is refused by name with the generated index rebuilt, and doctor --fix keeps a pointer that names an open session by alias (#376, #370, #337, #378; #375 and #338 await Leif's choice)

## Goal

Migration refuses before it writes and leaves a working tree: any entry at a target or receipt name counts as taken, parallel applies cannot both apply, a returning 0.5.x record already migrated is refused by name with the generated index rebuilt, and doctor --fix keeps a pointer that names an open session by alias (#376, #370, #337, #378; #375 and #338 await Leif's choice)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-m1-migrate-refuses-before-writing (native approval)": "Approve with the 3 choices" (Scope: his 'Approve with the 3 choices' of the draft at a82f0e4e (Claude Code chat message relayed by hc t4c through fable-lead, hc t53, 2026-11-19). The one path tests/fixtures/migrate/ added on his 'a, all yes.. im back' (relayed the same way, about 00:38Z on 2026-11-20, item 8). Recording confirmed by his direct answer 'Yes, record it' in t4b's Claude Code session, 2026-11-20); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-m1-migrate-refuses-before-writing as 0xLeif

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
- 2026-11-20T00:44:43Z · gate-opened · plan → plan · Scope approval of fix-m1-migrate-refuses-before-writing (native approval) ^t-6f005e
  - revision: a82f0e4e83c5cc8dbe3d42ea28f108df99281f0a
  - holds: plan
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:44:58Z · gate-closed · plan → plan · Scope approval of fix-m1-migrate-refuses-before-writing (native approval) ^t-b2a435
  - native_change: fix-m1-migrate-refuses-before-writing
  - revision: a82f0e4e83c5cc8dbe3d42ea28f108df99281f0a
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve with the 3 choices
    - where: Scope: his 'Approve with the 3 choices' of the draft at a82f0e4e (Claude Code chat message relayed by hc t4c through fable-lead, hc t53, 2026-11-19). The one path tests/fixtures/migrate/ added on his 'a, all yes.. im back' (relayed the same way, about 00:38Z on 2026-11-20, item 8). Recording confirmed by his direct answer 'Yes, record it' in t4b's Claude Code session, 2026-11-20
    - recorded_by: fable-t4b
    - native: specsync change approve fix-m1-migrate-refuses-before-writing
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
