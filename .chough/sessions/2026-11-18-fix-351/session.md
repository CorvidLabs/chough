---
kind: session
id: 2026-11-18-fix-351
aliases: [fix-351]
objective: The workflow skill teaches --no-native-change, and chough next offers both paths when a move is refused for lacking a native change (#351)
intent_scope: ""
modules: []
stage: test
stage_status: active
iteration: 1
callers: []
native_change: fix-351
pr: ""
criteria: [DRIVE-16]
requirements: [REQ-cli-021]
cases: [TC-cli-a-refused-move-for-lacking-a-change-offers-both-paths]
design: []
files: [src/stage.rs, tests/stages.rs, skills/chough-workflow/references/lifecycle.md]
observations: [2026-11-18-tc-cli-a-refused-move-for-lacking-a-change-offers-both-paths]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue test
---
# The workflow skill teaches --no-native-change, and chough next offers both paths when a move is refused for lacking a native change (#351)

## Goal

The workflow skill teaches --no-native-change, and chough next offers both paths when a move is refused for lacking a native change (#351)

## Where it stands

- The refusal and next name both paths and the skill teaches the flag; the test fails on main and passes with the change

## Decisions and why

- 0xLeif decided "Scope approval of fix-351": "yes, both" (Leif's decision of 2026-11-18 recorded in GitHub issue #351 (the skill explains chough checkpoint --no-native-change, and chough next offers both paths when a move is refused for lacking a native change); his planning session (hc t4c) relayed it to fable-t4b as his scope approval); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-351 as 0xLeif

## What was verified

- [Ran cargo test --locked --test stages -- --exact a_move_refused_for_lacking_a_change_offers_both_paths for TC-cli-a-refused-move-for-lacking-a-change-offers-both-paths: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-a-refused-move-for-lacking-a-change-offers-both-paths.md)

## Open threads

## Timeline

- 2026-11-18T22:53:36Z · start · intent → intent · The workflow skill teaches --no-native-change, and chough next offers both paths when a move is refused for lacking a native change (#351) ^t-abfe35
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:53:36Z · advance · intent → design ·  ^t-f3129c
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:53:36Z · skip · design → plan · Guidance text change; nothing visual or interactive changes ^t-186e68
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:54:48Z · gate-opened · plan → plan · Scope approval of fix-351 ^t-7e0946
  - native_change: fix-351
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:54:48Z · gate-closed · plan → plan · Scope approval of fix-351 ^t-09dda0
  - native_change: fix-351
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes, both
    - where: Leif's decision of 2026-11-18 recorded in GitHub issue #351 (the skill explains chough checkpoint --no-native-change, and chough next offers both paths when a move is refused for lacking a native change); his planning session (hc t4c) relayed it to fable-t4b as his scope approval
    - recorded_by: fable-t4b
    - native: specsync change approve fix-351
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:54:49Z · advance · plan → build · native:approved ^t-bd82ec
  - native_change: fix-351
  - gate: native:approved
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:54:49Z · advance · build → test · native:approved ^t-02147d
  - native_change: fix-351
  - gate: native:approved
  - revision: 235c3bfb063e835f67c4aae12f873ceac4074be7
  - by: fable (claude-fable-5-2, hc t4b)
