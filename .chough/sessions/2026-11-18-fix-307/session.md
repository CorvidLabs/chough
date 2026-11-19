---
kind: session
id: 2026-11-18-fix-307
aliases: [fix-307]
objective: "chough change ready names an affected path whose owning module the change does not declare, so the approval does not refuse on it after ready said nothing is missing (REQ-cli-067, #307)"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: fix-307
pr: ""
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
cases: []
design: []
files: [src/ready.rs, tests/ready.rs, docs/cli.md]
observations: [2026-11-18-tc-cli-change-ready-undeclared-owning-module]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Gate \"Scope approval of fix-307, rebased onto the change-ready wording (native approval)\" closed (approved); run `chough stage advance` to move on."
---
# chough change ready names an affected path whose owning module the change does not declare, so the approval does not refuse on it after ready said nothing is missing (REQ-cli-067, #307)

## Goal

chough change ready names an affected path whose owning module the change does not declare, so the approval does not refuse on it after ready said nothing is missing (REQ-cli-067, #307)

## Where it stands

- Implemented the undeclared owning module finding; tests/ready.rs passes 10 of 10

## Decisions and why

- A missing owning module is a new kind of finding, so REQ-cli-067 is modified by a delta; the finding names the module once with its paths and says declaring it can add questions, which the next report lists
- 0xLeif decided "Scope approval of fix-307": "Approve (Recommended)" (Claude Code session with Fable, 2026-11-18, answer to the interview question 'Do you approve the scope of fix-307?'); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-307 as 0xLeif
- The first scope named folders and test files that SpecSync's approval accepts; on Leif's choice 'You run it this once' fable-t4b ran specsync change send-back and narrowed the entry to SpecSync 6.3.0's own rule
- 0xLeif decided "Scope approval of fix-307": "Approve (Recommended)" (Claude Code session with Fable, 2026-11-18, answer to the interview question 'fix-307, corrected scope ... Do you approve this scope?' after the send-back); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-307 as 0xLeif
- 0xLeif decided "Scope approval of fix-307, rebased onto the change-ready wording (native approval)": "Yes, for today" (Leif's standing rule of 2026-11-18, relayed by his planning session (hc t4c): when an approved PR's delta goes stale only because the same requirement changed under it, and the rebase puts the same added sentences and criteria on the newer text with nothing else changed, fable-t4b may record his relayed re-approval and report the PR and requirement. Here: PR #310, REQ-cli-067, rebased onto #311's approved own-work wording); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-307 as 0xLeif

## What was verified

- [Ran cargo test --locked --test ready -- --exact a_path_whose_owning_module_is_not_declared_is_named for TC-cli-change-ready-undeclared-owning-module: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-change-ready-undeclared-owning-module.md)

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
- 2026-11-18T14:45:22Z · gate-closed · plan → plan · Scope approval of fix-307 ^t-de7db2
  - native_change: fix-307
  - revision: 3eab8e6fa14f5823c3ead5361ef5fa592bb1268f
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve (Recommended)
    - where: Claude Code session with Fable, 2026-11-18, answer to the interview question 'Do you approve the scope of fix-307?'
    - recorded_by: fable-t4b
    - native: specsync change approve fix-307
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T14:49:15Z · gate-opened · plan → plan · Scope approval of fix-307 ^t-e22621
  - native_change: fix-307
  - revision: 3eab8e6fa14f5823c3ead5361ef5fa592bb1268f
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T14:49:57Z · direct-native-write · plan → plan · fix-307's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-6f562f
  - native_change: fix-307
  - revision: 315aca55cae30ebf05f77d0af8dae048298e24ff
  - direct_write:
    - change: fix-307
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T14:49:57Z · gate-closed · plan → plan · Scope approval of fix-307 ^t-15ed57
  - native_change: fix-307
  - revision: 315aca55cae30ebf05f77d0af8dae048298e24ff
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve (Recommended)
    - where: Claude Code session with Fable, 2026-11-18, answer to the interview question 'fix-307, corrected scope ... Do you approve this scope?' after the send-back
    - recorded_by: fable-t4b
    - native: specsync change approve fix-307
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T14:53:16Z · advance · plan → build · native:approved ^t-6b6a61
  - native_change: fix-307
  - gate: native:approved
  - revision: 315aca55cae30ebf05f77d0af8dae048298e24ff
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T04:25:41Z · gate-opened · build → build · Scope approval of fix-307, rebased onto the change-ready wording (native approval) ^t-494f18
  - native_change: fix-307
  - revision: 8ff522b62f3767e813890743f049150572c526a1
  - holds: build
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T04:25:41Z · direct-native-write · build → build · fix-307's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-fdb3e7
  - native_change: fix-307
  - revision: 8ff522b62f3767e813890743f049150572c526a1
  - direct_write:
    - change: fix-307
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T04:25:41Z · gate-closed · build → build · Scope approval of fix-307, rebased onto the change-ready wording (native approval) ^t-3d50f4
  - native_change: fix-307
  - revision: 8ff522b62f3767e813890743f049150572c526a1
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, for today
    - where: Leif's standing rule of 2026-11-18, relayed by his planning session (hc t4c): when an approved PR's delta goes stale only because the same requirement changed under it, and the rebase puts the same added sentences and criteria on the newer text with nothing else changed, fable-t4b may record his relayed re-approval and report the PR and requirement. Here: PR #310, REQ-cli-067, rebased onto #311's approved own-work wording
    - recorded_by: fable-t4b
    - native: specsync change approve fix-307
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
