---
kind: session
id: 2026-11-18-blocking-bug-session
aliases: [blocking-bug-session]
objective: A blocking bug stops only a session it is related to
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: a-blocking-bug-stops-only-a-session-it-is-related-to
pr: ""
criteria: [FLOW-8]
requirements: [REQ-cli-054, REQ-cli-021, REQ-cli-052]
cases: []
design: []
files: []
observations:
  - 2026-11-18-cargo-test-locked-test-bugs-advance-to-and-skill-releases
  - 2026-11-18-tc-cli-bug-only-open-blocking-blocks
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
title: A blocking bug stops only its session
summary:
  - A blocking bug stops a session only when that session lists it, its evidence names the session, or they share a criterion, requirement, or case. Other sessions are warned by name. A bug that names nothing stops no session.
  - cargo test --locked for bugs, advance_to, and skill_releases passed. TC-cli-bug-only-open-blocking-blocks passed on local-macos-arm64. The full verify lane was not run.
  - Implementation review is the remaining human gate. Shipping archives the change so CI can pass the archive gate.
---
# A blocking bug stops only a session it is related to

## Goal

A blocking bug stops only a session it is related to

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of a blocking bug stops only a session it is related to": "i approve" (reply in this chat on 2026-11-18 to the blocking-bug rule); recorded by grok-t1w; a relayed claim, not authenticated; ran specsync change approve a-blocking-bug-stops-only-a-session-it-is-related-to as 0xLeif
- 0xLeif decided "Implementation review of a-blocking-bug-stops-only-a-session-it-is-related-to": "i approve" (reply in this chat to the implementation review); recorded by grok-t1w; a relayed claim, not authenticated; ran specsync change review a-blocking-bug-stops-only-a-session-it-is-related-to as 0xLeif with verdict pass

## What was verified

- [cargo test --locked --test bugs, advance_to, and skill_releases passed, plus the linking-bug stages test and the large next-view test. The new two-session test passed after it had failed on the repo-wide blocking-bug refusal.](evidence/2026-11-18-cargo-test-locked-test-bugs-advance-to-and-skill-releases.md)
- [Ran cargo test --locked --test bugs -- --exact only_an_open_blocking_bug_blocks_leaving_test for TC-cli-bug-only-open-blocking-blocks: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-bug-only-open-blocking-blocks.md)

## Open threads

## Timeline

- 2026-11-18T00:24:53Z · start · intent → intent · A blocking bug stops only a session it is related to ^t-e3527b
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: grok-t1w (grok, hc t1w)
- 2026-11-18T00:24:53Z · advance · intent → design ·  ^t-8ba3ab
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: grok-t1w (grok, hc t1w)
- 2026-11-18T00:24:53Z · skip · design → plan · No visual or interaction change ^t-ad087e
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: grok-t1w (grok, hc t1w)
- 2026-11-18T00:26:25Z · gate-opened · plan → plan · Scope approval of a blocking bug stops only a session it is related to ^t-fdda16
  - native_change: a-blocking-bug-stops-only-a-session-it-is-related-to
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - holds: plan
  - drives: approve
  - by: grok-t1w (grok, hc t1w)
- 2026-11-18T00:26:25Z · gate-closed · plan → plan · Scope approval of a blocking bug stops only a session it is related to ^t-2f9f3b
  - native_change: a-blocking-bug-stops-only-a-session-it-is-related-to
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - relayed_by:
    - decided_by: 0xLeif
    - words: i approve
    - where: reply in this chat on 2026-11-18 to the blocking-bug rule
    - recorded_by: grok-t1w
    - native: specsync change approve a-blocking-bug-stops-only-a-session-it-is-related-to
    - actor: 0xLeif
  - by: grok-t1w (grok, hc t1w)
- 2026-11-18T00:32:06Z · advance · plan → build · native:verifying ^t-74c2e3
  - native_change: a-blocking-bug-stops-only-a-session-it-is-related-to
  - gate: native:verifying
  - revision: 7bb2ffc7a8579bd0d69f47b0040d03d9ea36d8c3
  - by: grok-t1w (grok, hc t1w)
- 2026-11-18T00:32:06Z · advance · build → test · native:verifying ^t-698c4f
  - native_change: a-blocking-bug-stops-only-a-session-it-is-related-to
  - gate: native:verifying
  - revision: 7bb2ffc7a8579bd0d69f47b0040d03d9ea36d8c3
  - by: grok-t1w (grok, hc t1w)
- 2026-11-18T00:32:44Z · advance · test → accept · native:verifying ^t-3848c2
  - native_change: a-blocking-bug-stops-only-a-session-it-is-related-to
  - gate: native:verifying
  - revision: 7bb2ffc7a8579bd0d69f47b0040d03d9ea36d8c3
  - by: grok-t1w (grok, hc t1w)
- 2026-11-18T00:33:40Z · gate-opened · accept → accept · Implementation review of a-blocking-bug-stops-only-a-session-it-is-related-to ^t-889987
  - native_change: a-blocking-bug-stops-only-a-session-it-is-related-to
  - revision: 7bb2ffc7a8579bd0d69f47b0040d03d9ea36d8c3
  - holds: accept
  - drives: review
  - by: grok-t1w (grok, hc t1w)
- 2026-11-18T00:36:39Z · gate-closed · accept → accept · Implementation review of a-blocking-bug-stops-only-a-session-it-is-related-to ^t-064943
  - native_change: a-blocking-bug-stops-only-a-session-it-is-related-to
  - revision: becc44ec2bcdeb7dca08df51b11e99900ae55049
  - relayed_by:
    - decided_by: 0xLeif
    - words: i approve
    - where: reply in this chat to the implementation review
    - recorded_by: grok-t1w
    - summary_digest: f78ad202476dc469ed9bab80f7d8c76e0885071ef5fb61912783bb03ffaba12c
    - list: 8f34b6bbd278
    - native: specsync change review a-blocking-bug-stops-only-a-session-it-is-related-to
    - actor: 0xLeif
  - by: grok-t1w (grok, hc t1w)
- 2026-11-18T00:36:40Z · advance · accept → done · native:archived ^t-78b63e
  - native_change: a-blocking-bug-stops-only-a-session-it-is-related-to
  - gate: native:archived
  - revision: becc44ec2bcdeb7dca08df51b11e99900ae55049
  - by: grok-t1w (grok, hc t1w)
