---
kind: session
id: 2026-11-18-fix-277
aliases: [fix-277]
objective: "A migrated bug's body links each carried observation to the file of the entry or story that holds it (REQ-records-009, #277)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-277
pr: ""
criteria: [LIFE-20]
requirements: [REQ-records-009]
cases: [TC-records-a-migrated-bugs-body-links-its-carried-observations]
design: []
files: [src/bugs.rs, src/migrate.rs, tests/migrate_evidence.rs]
observations:
  - 2026-11-18-tc-records-a-migrated-bugs-body-links-its-carried-observations
  - 2026-11-18-tc-records-a-migrated-bugs-body-links-its-carried-observations-1d1109
  - 2026-11-18-tc-records-a-migrated-bugs-body-links-its-carried-observations-af96fe
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-277
    holds: accept
    drives: review
    opened: 2026-11-18T19:55:31Z
run_consents: []
next: Wait for the tester's check and Leif's implementation review of fix-277
title: A migrated bug's Log links each carried observation
summary:
  - A bug that chough migrate writes named its carried observations in its Log by bare 0.5.x ID (#277). The Log now links each to the open session's evidence entry or to the archived story that holds its receipt, relative to the bug's file; a receipt only the bug holds is named as its own. A new test failed on main first; the verify lane passed on macOS after main was merged in, and the tester passed it. Linux was not run locally.
---
# A migrated bug's body links each carried observation to the file of the entry or story that holds it (REQ-records-009, #277)

## Goal

A migrated bug's body links each carried observation to the file of the entry or story that holds it (REQ-records-009, #277)

## Where it stands

- A migrated bug's Log links each carried observation; the tests fail on main and pass with the fix

## Decisions and why

- 0xLeif decided "Scope approval of fix-277": "relative links. A migrated bug's body links each carried observation to the file of the entry or story that holds it, as a new bug's body does." (Leif's decision comment on GitHub issue #277, 2026-11-18, from his planning interview; his planning session (hc t4c) relayed it to fable-t4b as his scope approval); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-277 as 0xLeif

## What was verified

- [Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_bugs_body_links_its_carried_observations for TC-records-a-migrated-bugs-body-links-its-carried-observations: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-migrated-bugs-body-links-its-carried-observations.md)
- [Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_bugs_body_links_its_carried_observations for TC-records-a-migrated-bugs-body-links-its-carried-observations: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-migrated-bugs-body-links-its-carried-observations-1d1109.md)
- [Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_bugs_body_links_its_carried_observations for TC-records-a-migrated-bugs-body-links-its-carried-observations: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-migrated-bugs-body-links-its-carried-observations-af96fe.md)

## Open threads

## Timeline

- 2026-11-18T19:43:39Z · start · intent → intent · A migrated bug's body links each carried observation to the file of the entry or story that holds it (REQ-records-009, #277) ^t-5d8ad0
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:43:39Z · advance · intent → design ·  ^t-5fc35c
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:43:39Z · skip · design → plan · Migration output change; nothing visual or interactive changes ^t-e2b287
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:46:22Z · gate-opened · plan → plan · Scope approval of fix-277 ^t-b2d1d7
  - native_change: fix-277
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:46:22Z · gate-closed · plan → plan · Scope approval of fix-277 ^t-d509b5
  - native_change: fix-277
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - relayed_by:
    - decided_by: 0xLeif
    - words: relative links. A migrated bug's body links each carried observation to the file of the entry or story that holds it, as a new bug's body does.
    - where: Leif's decision comment on GitHub issue #277, 2026-11-18, from his planning interview; his planning session (hc t4c) relayed it to fable-t4b as his scope approval
    - recorded_by: fable-t4b
    - native: specsync change approve fix-277
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:46:23Z · advance · plan → build · native:approved ^t-8beb44
  - native_change: fix-277
  - gate: native:approved
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:46:24Z · advance · build → test · native:approved ^t-3fc480
  - native_change: fix-277
  - gate: native:approved
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:55:31Z · advance · test → accept · native:verifying ^t-3a8c08
  - native_change: fix-277
  - gate: native:verifying
  - revision: 50aa2a4988e3d786aefa2641b6636e28771db80c
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:55:31Z · gate-opened · accept → accept · Implementation review of fix-277 ^t-57502e
  - native_change: fix-277
  - revision: 50aa2a4988e3d786aefa2641b6636e28771db80c
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
