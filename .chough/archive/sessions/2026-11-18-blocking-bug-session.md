---
kind: session
id: 2026-11-18-blocking-bug-session
aliases: [blocking-bug-session]
objective: A blocking bug stops only a session it is related to
stage: done
stage_status: archived
iteration: 1
native_change: a-blocking-bug-stops-only-a-session-it-is-related-to
criteria: [FLOW-8]
requirements: [REQ-cli-054, REQ-cli-021, REQ-cli-052]
observations:
  - 2026-11-18-cargo-test-locked-test-bugs-advance-to-and-skill-releases
  - 2026-11-18-tc-cli-bug-only-open-blocking-blocks
title: A blocking bug stops only its session
summary:
  - A blocking bug stops a session only when that session lists it, its evidence names the session, or they share a criterion, requirement, or case. Other sessions are warned by name. A bug that names nothing stops no session.
  - cargo test --locked for bugs, advance_to, and skill_releases passed. TC-cli-bug-only-open-blocking-blocks passed on local-macos-arm64. The full verify lane was not run.
  - Implementation review is the remaining human gate. Shipping archives the change so CI can pass the archive gate.
closed: 2026-11-18
outcome: done
---
# A blocking bug stops only a session it is related to

## Goal

A blocking bug stops only a session it is related to

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of a blocking bug stops only a session it is related to": "i approve" (reply in this chat on 2026-11-18 to the blocking-bug rule); recorded by grok-t1w; a relayed claim, not authenticated; ran specsync change approve a-blocking-bug-stops-only-a-session-it-is-related-to as 0xLeif
- 0xLeif decided "Implementation review of a-blocking-bug-stops-only-a-session-it-is-related-to": "i approve" (reply in this chat to the implementation review); recorded by grok-t1w; a relayed claim, not authenticated; ran specsync change review a-blocking-bug-stops-only-a-session-it-is-related-to as 0xLeif with verdict pass

## What was verified

- cargo test --locked --test bugs, advance_to, and skill_releases passed, plus the linking-bug stages test and the large next-view test. The new two-session test passed after it had failed on the repo-wide blocking-bug refusal. ^2026-11-18-cargo-test-locked-test-bugs-advance-to-and-skill-releases
- Ran cargo test --locked --test bugs -- --exact only_an_open_blocking_bug_blocks_leaving_test for TC-cli-bug-only-open-blocking-blocks: passed (exit 0, tests run 1). - 5da783ad34cf ^2026-11-18-tc-cli-bug-only-open-blocking-blocks

## Bugs

None.

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

Closed 2026-11-18: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-18-cargo-test-locked-test-bugs-advance-to-and-skill-releases
    claim: cargo test --locked --test bugs, advance_to, and skill_releases passed, plus the linking-bug stages test and the large next-view test. The new two-session test passed after it had failed on the repo-wide blocking-bug refusal.
    result: passed
    revision: 7bb2ffc7a8579bd0d69f47b0040d03d9ea36d8c3
    environment: local
    content: 960c6d0f19fc3725c7582da2d7b299176a839ec9d092f2e909f35bc1cfa3dc16
    observed: 2026-11-18T00:32:12Z
    actor: grok-t1w (grok, hc t1w)
  - id: 2026-11-18-tc-cli-bug-only-open-blocking-blocks
    claim: "Ran cargo test --locked --test bugs -- --exact only_an_open_blocking_bug_blocks_leaving_test for TC-cli-bug-only-open-blocking-blocks: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact only_an_open_blocking_bug_blocks_leaving_test
    revision: 7bb2ffc7a8579bd0d69f47b0040d03d9ea36d8c3
    environment: local-macos-arm64
    content: 960c6d0f19fc3725c7582da2d7b299176a839ec9d092f2e909f35bc1cfa3dc16
    fingerprint: sha256:5da783ad34cf59242f7196542377b8f9a4e732be74a5b53a9f84a8c57c1b507a
    case: TC-cli-bug-only-open-blocking-blocks
    case_digest: c9dafba98df618115f41b036fccb2b66d1057cbaf9d0ea49346aac76953a0423
    observed: 2026-11-18T00:33:33Z
    duration_seconds: 3
    actor: grok-t1w (grok, hc t1w)
```
