---
kind: session
id: 2026-11-18-fix-351
aliases: [fix-351]
objective: The workflow skill teaches --no-native-change, and chough next offers both paths when a move is refused for lacking a native change (#351)
intent_scope: ""
modules: []
stage: accept
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
observations:
  - 2026-11-18-tc-cli-a-refused-move-for-lacking-a-change-offers-both-paths
  - 2026-11-18-tc-cli-a-refused-move-for-lacking-a-change-offers-both-paths-8e78d9
  - 2026-11-19-tc-cli-a-refused-move-for-lacking-a-change-offers-both-paths
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-351
    holds: accept
    drives: review
    opened: 2026-11-18T22:58:59Z
run_consents: []
next: Wait for the tester's check and Leif's implementation review of fix-351
title: next and the skill teach --no-native-change
summary:
  - Nothing told an agent that chough checkpoint --no-native-change exists (#351). A move refused for lacking a native change, and chough next, now name both ways on; once the flag is set next says the session needs no native change, with the reason; the literal <reason> placeholder is refused. The workflow skill explains the flag, and a project installed by 0.6.1 gets the new text on chough skill update. Tests failed first; the verify lane passed on macOS after main was merged in, and the tester passed the recheck. Linux was not run locally.
---
# The workflow skill teaches --no-native-change, and chough next offers both paths when a move is refused for lacking a native change (#351)

## Goal

The workflow skill teaches --no-native-change, and chough next offers both paths when a move is refused for lacking a native change (#351)

## Where it stands

- The refusal and next name both paths and the skill teaches the flag; the test fails on main and passes with the change

## Decisions and why

- 0xLeif decided "Scope approval of fix-351": "yes, both" (Leif's decision of 2026-11-18 recorded in GitHub issue #351 (the skill explains chough checkpoint --no-native-change, and chough next offers both paths when a move is refused for lacking a native change); his planning session (hc t4c) relayed it to fable-t4b as his scope approval); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-351 as 0xLeif
- 0xLeif decided "Scope approval of fix-351, with three more paths (native approval)": "Yes: send back once + re-approve" (Relayed by Leif's planning session (hc t4c) on 2026-11-18, which asked him fable-t4b's question: send back fix-351 once and re-approve it with src/skill.rs, src/checkpoint.rs and src/main.rs added; the requirement text does not change); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-351 as 0xLeif

## What was verified

- [Ran cargo test --locked --test stages -- --exact a_move_refused_for_lacking_a_change_offers_both_paths for TC-cli-a-refused-move-for-lacking-a-change-offers-both-paths: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-a-refused-move-for-lacking-a-change-offers-both-paths.md)
- [Ran cargo test --locked --test stages -- --exact a_move_refused_for_lacking_a_change_offers_both_paths for TC-cli-a-refused-move-for-lacking-a-change-offers-both-paths: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-a-refused-move-for-lacking-a-change-offers-both-paths-8e78d9.md)
- [Ran cargo test --locked --test stages -- --exact a_move_refused_for_lacking_a_change_offers_both_paths for TC-cli-a-refused-move-for-lacking-a-change-offers-both-paths: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-a-refused-move-for-lacking-a-change-offers-both-paths.md)

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
- 2026-11-18T22:58:59Z · advance · test → accept · native:verifying ^t-353f53
  - native_change: fix-351
  - gate: native:verifying
  - revision: 6d0d9165e59d5f1c668304e3dd2102c23578a343
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T22:58:59Z · gate-opened · accept → accept · Implementation review of fix-351 ^t-8df6a1
  - native_change: fix-351
  - revision: 6d0d9165e59d5f1c668304e3dd2102c23578a343
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:17:22Z · gate-opened · accept → accept · Scope approval of fix-351, with three more paths (native approval) ^t-fc1e4e
  - native_change: fix-351
  - revision: 4e9b834f753512414f83850ff174074ace676328
  - holds: accept
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:17:22Z · direct-native-write · accept → accept · fix-351's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-8a1656
  - native_change: fix-351
  - revision: 4e9b834f753512414f83850ff174074ace676328
  - direct_write:
    - change: fix-351
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:17:22Z · gate-closed · accept → accept · Scope approval of fix-351, with three more paths (native approval) ^t-761d73
  - native_change: fix-351
  - revision: 4e9b834f753512414f83850ff174074ace676328
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes: send back once + re-approve
    - where: Relayed by Leif's planning session (hc t4c) on 2026-11-18, which asked him fable-t4b's question: send back fix-351 once and re-approve it with src/skill.rs, src/checkpoint.rs and src/main.rs added; the requirement text does not change
    - recorded_by: fable-t4b
    - native: specsync change approve fix-351
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
