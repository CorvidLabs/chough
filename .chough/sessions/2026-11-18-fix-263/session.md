---
kind: session
id: 2026-11-18-fix-263
aliases: [fix-263]
objective: "migrate --apply refuses before writing when a planned target that differs only in case from a removed 0.5.x file already exists with other bytes (REQ-records-009, #263)"
intent_scope: ""
modules: []
stage: test
stage_status: active
iteration: 1
callers: []
native_change: fix-263
pr: ""
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
cases: [TC-records-migrate-refuses-a-separate-case-colliding-target]
design: []
files: [src/migrate.rs, tests/migrate_knowledge.rs]
observations: [2026-11-18-tc-records-migrate-refuses-a-separate-case-colliding-target]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue test
---
# migrate --apply refuses before writing when a planned target that differs only in case from a removed 0.5.x file already exists with other bytes (REQ-records-009, #263)

## Goal

migrate --apply refuses before writing when a planned target that differs only in case from a removed 0.5.x file already exists with other bytes (REQ-records-009, #263)

## Where it stands

- The pre-write check refuses a separate case-colliding target; the test fails on main and passes with the fix in chough-linux:3

## Decisions and why

- The pre-write check exempts a case-colliding target only when it is the removed file under its other spelling: the other spelling still opens and the folder does not list both names. The session's criteria list DRIVE-27 by my mistake; LIFE-20 is the one that applies
- 0xLeif decided "Scope approval of fix-263": "Approve (Recommended)" (Claude Code session with Fable, 2026-11-18, answer to the interview question 'fix-263 (draft PR #312) ... Do you approve the scope?'); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-263 as 0xLeif

## What was verified

- [Ran cargo test --locked --test migrate_knowledge -- --exact a_separate_file_at_a_case_colliding_target_refuses_before_any_write for TC-records-migrate-refuses-a-separate-case-colliding-target: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-migrate-refuses-a-separate-case-colliding-target.md)

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
- 2026-11-18T15:23:12Z · gate-closed · plan → plan · Scope approval of fix-263 ^t-e8f55d
  - native_change: fix-263
  - revision: 500ace1de42eff30f9fee1dac0f1a0ac5769dc51
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve (Recommended)
    - where: Claude Code session with Fable, 2026-11-18, answer to the interview question 'fix-263 (draft PR #312) ... Do you approve the scope?'
    - recorded_by: fable-t4b
    - native: specsync change approve fix-263
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T15:23:14Z · advance · plan → build · native:approved ^t-e3fbd4
  - native_change: fix-263
  - gate: native:approved
  - revision: 500ace1de42eff30f9fee1dac0f1a0ac5769dc51
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T15:23:14Z · advance · build → test · native:approved ^t-de6ee1
  - native_change: fix-263
  - gate: native:approved
  - revision: 500ace1de42eff30f9fee1dac0f1a0ac5769dc51
  - by: fable (claude-fable-5-2, hc t4b)
