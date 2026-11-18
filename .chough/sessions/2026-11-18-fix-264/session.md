---
kind: session
id: 2026-11-18-fix-264
aliases: [fix-264]
objective: "A second same-day migration names its receipt with a deterministic suffix that is part of the plan and its digest, and migrate --apply checks every target before it writes anything (REQ-records-009, #264)"
intent_scope: ""
modules: []
stage: test
stage_status: active
iteration: 1
callers: []
native_change: fix-264
pr: ""
criteria: [LIFE-20, DRIVE-27]
requirements: [REQ-records-009]
cases:
  - TC-records-a-second-same-day-migration-names-its-own-receipt
  - TC-records-a-receipt-name-taken-after-the-preview-refuses-before-any-write
design: []
files: [src/migrate.rs, tests/migrate.rs, docs/cli.md]
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue test
---
# A second same-day migration names its receipt with a deterministic suffix that is part of the plan and its digest, and migrate --apply checks every target before it writes anything (REQ-records-009, #264)

## Goal

A second same-day migration names its receipt with a deterministic suffix that is part of the plan and its digest, and migrate --apply checks every target before it writes anything (REQ-records-009, #264)

## Where it stands

- The plan names its receipt with a same-day suffix and the apply checks it before writing; both new tests fail on main and pass with the fix

## Decisions and why

- 0xLeif decided "Scope approval of fix-264": "suffix and check first. A second same-day migration plan names its receipt with a deterministic suffix (...-from-0-5-2.md), part of the plan and its digest, and --apply checks every target before it writes anything, so a refusal never leaves a partial write." (Leif's decision comment on GitHub issue #264, 2026-11-18, from his planning interview; his planning session (hc t4c) relayed it to fable-t4b as his scope approval); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-264 as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-18T19:33:35Z · start · intent → intent · A second same-day migration names its receipt with a deterministic suffix that is part of the plan and its digest, and migrate --apply checks every target before it writes anything (REQ-records-009, #264) ^t-b90353
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:33:35Z · advance · intent → design ·  ^t-51f942
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:33:35Z · skip · design → plan · Migration naming change; nothing visual or interactive changes ^t-83a0ba
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:36:00Z · gate-opened · plan → plan · Scope approval of fix-264 ^t-956fbb
  - native_change: fix-264
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:36:00Z · gate-closed · plan → plan · Scope approval of fix-264 ^t-a261de
  - native_change: fix-264
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - relayed_by:
    - decided_by: 0xLeif
    - words: suffix and check first. A second same-day migration plan names its receipt with a deterministic suffix (...-from-0-5-2.md), part of the plan and its digest, and --apply checks every target before it writes anything, so a refusal never leaves a partial write.
    - where: Leif's decision comment on GitHub issue #264, 2026-11-18, from his planning interview; his planning session (hc t4c) relayed it to fable-t4b as his scope approval
    - recorded_by: fable-t4b
    - native: specsync change approve fix-264
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:36:09Z · advance · plan → build · native:approved ^t-af0e8e
  - native_change: fix-264
  - gate: native:approved
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:36:09Z · advance · build → test · native:approved ^t-ee0199
  - native_change: fix-264
  - gate: native:approved
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
