---
kind: session
id: 2026-11-23-fix-440
aliases: [fix-440]
objective: "Fix chough#440: the stage advance --to go-back hint must name --reason like every other back/reset hint"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-440
pr: ""
criteria: [FLOW-2]
requirements: []
cases: [TC-cli-advance-to-not-ahead-hint-names-reason]
design: []
files: []
observations: [2026-11-23-tc-cli-advance-to-not-ahead-hint-names-reason]
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-440
    holds: accept
    drives: review
    opened: 2026-11-23T14:15:52Z
run_consents: []
next: Continue accept
---
# Fix chough#440: the stage advance --to go-back hint must name --reason like every other back/reset hint

## Goal

Fix chough#440: the stage advance --to go-back hint must name --reason like every other back/reset hint

## Where it stands

## Decisions and why

- 0xLeif decided "scope approval for fix-440: the stage advance --to go-back hint must name --reason like every other back/reset hint": "I approve" (pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch); recorded by pi-t14z; a relayed claim, not authenticated
- 0xLeif decided "Approve the scope of fix-440": "I approve" (pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change approve fix-440 as 0xLeif

## What was verified

- [Ran cargo test --locked --test advance_to -- --exact a_refused_step_refuses_the_whole_batch_with_nothing_recorded for TC-cli-advance-to-not-ahead-hint-names-reason: passed (exit 0).](evidence/2026-11-23-tc-cli-advance-to-not-ahead-hint-names-reason.md)

## Open threads

## Timeline

- 2026-11-23T05:16:40Z · start · intent → intent · fix-440 ^t-9a5151
  - revision: 0d5988f804155e7b3c285cace6cc0c52f540e035
  - by: unknown agent (hc t14z)
- 2026-11-23T14:03:41Z · gate-opened · intent → intent · scope approval for fix-440: the stage advance --to go-back hint must name --reason like every other back/reset hint ^t-9fccbe
  - revision: 0d5988f804155e7b3c285cace6cc0c52f540e035
  - holds: intent
  - drives: none
  - by: unknown agent (hc t14z)
- 2026-11-23T14:03:44Z · gate-closed · intent → intent · scope approval for fix-440: the stage advance --to go-back hint must name --reason like every other back/reset hint ^t-4b3d21
  - revision: 0d5988f804155e7b3c285cace6cc0c52f540e035
  - relayed_by:
    - decided_by: 0xLeif
    - words: I approve
    - where: pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch
    - recorded_by: pi-t14z
    - summary_digest: 9fb772c7a80bf09f796cbe5f3d0d10a9d8efaa3bff50d386c60e9bb42113593b
    - list: 21704d166f73
  - by: unknown agent (hc t14z)
- 2026-11-23T14:03:48Z · advance · intent → design ·  ^t-41cb83
  - revision: 0d5988f804155e7b3c285cace6cc0c52f540e035
  - by: unknown agent (hc t14z)
- 2026-11-23T14:03:48Z · advance · design → plan ·  ^t-1bb63a
  - revision: 0d5988f804155e7b3c285cace6cc0c52f540e035
  - by: unknown agent (hc t14z)
- 2026-11-23T14:03:55Z · gate-opened · plan → plan · Approve the scope of fix-440 ^t-ff35c6
  - native_change: fix-440
  - revision: 0d5988f804155e7b3c285cace6cc0c52f540e035
  - holds: plan
  - drives: approve
  - by: unknown agent (hc t14z)
- 2026-11-23T14:03:57Z · direct-native-write · plan → plan · fix-440's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-0458c4
  - native_change: fix-440
  - revision: 0d5988f804155e7b3c285cace6cc0c52f540e035
  - direct_write:
    - change: fix-440
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: unknown agent (hc t14z)
- 2026-11-23T14:03:57Z · gate-closed · plan → plan · Approve the scope of fix-440 ^t-cde4f9
  - native_change: fix-440
  - revision: 0d5988f804155e7b3c285cace6cc0c52f540e035
  - relayed_by:
    - decided_by: 0xLeif
    - words: I approve
    - where: pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch
    - recorded_by: pi-t14z
    - summary_digest: 7490b5b5f39e1f0d647f273f96377b654f76d0a70e5ba1cf62b386a38589392f
    - list: 27771af38ae1
    - native: specsync change approve fix-440
    - actor: 0xLeif
  - by: unknown agent (hc t14z)
- 2026-11-23T14:15:45Z · advance · plan → build · native:approved ^t-4689bc
  - native_change: fix-440
  - gate: native:approved
  - revision: 6fd77b85ab3c24501f1f818e8b230d34335c067c
  - by: unknown agent (hc t14z)
- 2026-11-23T14:15:46Z · advance · build → test · native:approved ^t-776ba6
  - native_change: fix-440
  - gate: native:approved
  - revision: 6fd77b85ab3c24501f1f818e8b230d34335c067c
  - by: unknown agent (hc t14z)
- 2026-11-23T14:15:48Z · advance · test → accept · native:approved ^t-78979a
  - native_change: fix-440
  - gate: native:approved
  - revision: 6fd77b85ab3c24501f1f818e8b230d34335c067c
  - by: unknown agent (hc t14z)
- 2026-11-23T14:15:52Z · gate-opened · accept → accept · Implementation review of fix-440 ^t-7dd89c
  - native_change: fix-440
  - revision: 6fd77b85ab3c24501f1f818e8b230d34335c067c
  - holds: accept
  - drives: review
  - by: unknown agent (hc t14z)
