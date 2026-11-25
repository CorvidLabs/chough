---
kind: session
id: 2026-11-25-the-chough-workflow-skill-routes-every-lifecycle-writer
aliases: [the-chough-workflow-skill-routes-every-lifecycle-writer-through]
objective: "The chough workflow skill routes every lifecycle writer through chough subcommands: direct specsync change invocation becomes a recorded violation with the chough-first order explicit"
intent_scope: process
modules: [workflow]
stage: done
stage_status: archived
iteration: 1
native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
criteria: [DRIVE-20]
closed: 2026-11-25
outcome: done
---
# The chough workflow skill routes every lifecycle writer through chough subcommands: direct specsync change invocation becomes a recorded violation with the chough-first order explicit

## Goal

The chough workflow skill routes every lifecycle writer through chough subcommands: direct specsync change invocation becomes a recorded violation with the chough-first order explicit

## Where it stands

## Decisions and why

- Align the chough-workflow skill with DRIVE-20: the skill routes every lifecycle writer through chough subcommands; direct specsync change invocation is a recorded violation. Reason: the skill's 'Run HI, SpecSync and Fledge yourself' wording invited the PR-38 mistake where the orchestrator drove specsync directly and the chough session trail could not finish done (source: Leif, pi chat t14z, 2026-11-25, choosing 'Fix the skill + rules')
- leif decided "Approve the scope of align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct": "Approve scope (Recommended)" (pi chat t14z interview, 2026-11-25T16:45Z (question: Chough change REQ-cli-072 (skill routes all lifecycle writers through chough): approve scope?)); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change approve align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct as 0xLeif
- leif decided "Implementation review of align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct": "I approve" (pi chat t14z, 2026-11-25T17:20Z (Leif's own message in the orchestrator chat, after the review round and the fix commit)); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change review align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct as 0xLeif with verdict pass

## What was verified

## Bugs

None.

## Open threads

## Timeline

- 2026-11-25T16:34:02Z · start · intent → intent · The chough workflow skill routes every lifecycle writer through chough subcommands: direct specsync change invocation becomes a recorded violation with the chough-first order explicit ^t-7ac95d
  - revision: 050e2c35b5f4f1f7c16d45e66ca697597803fb80
  - by: unknown agent (hc t14z)
- 2026-11-25T16:34:21Z · advance · intent → design ·  ^t-1b5980
  - revision: 050e2c35b5f4f1f7c16d45e66ca697597803fb80
  - by: unknown agent (hc t14z)
- 2026-11-25T16:34:41Z · advance · design → plan ·  ^t-27b16c
  - revision: 050e2c35b5f4f1f7c16d45e66ca697597803fb80
  - by: unknown agent (hc t14z)
- 2026-11-25T16:36:47Z · gate-opened · plan → plan · Approve the scope of align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct ^t-de47ab
  - native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
  - revision: 050e2c35b5f4f1f7c16d45e66ca697597803fb80
  - holds: plan
  - drives: approve
  - by: unknown agent (hc t14z)
- 2026-11-25T16:37:14Z · gate-closed · plan → plan · Approve the scope of align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct ^t-c96fc0
  - native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
  - revision: 050e2c35b5f4f1f7c16d45e66ca697597803fb80
  - relayed_by:
    - decided_by: leif
    - words: Approve scope (Recommended)
    - where: pi chat t14z interview, 2026-11-25T16:45Z (question: Chough change REQ-cli-072 (skill routes all lifecycle writers through chough): approve scope?)
    - recorded_by: pi-t14z
    - native: specsync change approve align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
    - actor: 0xLeif
  - by: unknown agent (hc t14z)
- 2026-11-25T16:37:19Z · advance · plan → build · native:approved ^t-5d3b32
  - native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
  - gate: native:approved
  - revision: 050e2c35b5f4f1f7c16d45e66ca697597803fb80
  - by: unknown agent (hc t14z)
- 2026-11-25T16:39:18Z · advance · build → test · native:verifying ^t-cf5bf4
  - native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
  - gate: native:verifying
  - revision: ab567c99a6d4fa2e8e74d296f5fcc957d4c47bd2
  - by: unknown agent (hc t14z)
- 2026-11-25T16:39:19Z · advance · test → accept · native:verifying ^t-92517f
  - native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
  - gate: native:verifying
  - revision: ab567c99a6d4fa2e8e74d296f5fcc957d4c47bd2
  - by: unknown agent (hc t14z)
- 2026-11-25T16:39:39Z · gate-opened · accept → accept · Implementation review of align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct ^t-4786fb
  - native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
  - revision: ce544ec81e29d32c0a879c43427355603c962b48
  - holds: accept
  - drives: review
  - by: unknown agent (hc t14z)
- 2026-11-25T16:51:18Z · correction · accept → accept · Citation time corrected: the interview question was issued at ~2026-11-25T16:30Z and answered before the 16:37:14Z recording; the note's 16:45Z was written before the question was asked and overstated the delay. Leif's words and the question text are unchanged (reviewer L1). ^t-3a7eee
  - corrects: ^t-c96fc0
  - by: unknown agent (hc t14z)
- 2026-11-25T18:08:17Z · direct-native-write · accept → accept · align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct's lifecycle records changed outside Chough: approvals.json changed; state stays verifying ^t-6182a6
  - native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
  - revision: b1f96755d7205b67d029439edf3c1bbf2933203d
  - direct_write:
    - change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
    - files:
      - approvals.json: changed
    - before_state: verifying
    - after_state: verifying
  - by: unknown agent (hc t14z)
- 2026-11-25T18:08:17Z · gate-closed · accept → accept · Implementation review of align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct ^t-d0104d
  - native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
  - revision: b1f96755d7205b67d029439edf3c1bbf2933203d
  - relayed_by:
    - decided_by: leif
    - words: I approve
    - where: pi chat t14z, 2026-11-25T17:20Z (Leif's own message in the orchestrator chat, after the review round and the fix commit)
    - recorded_by: pi-t14z
    - native: specsync change review align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
    - actor: 0xLeif
  - by: unknown agent (hc t14z)
- 2026-11-25T18:10:25Z · direct-native-write · accept → accept · align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct's lifecycle records changed outside Chough: approvals.json changed; state stays verifying ^t-b5274e
  - native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
  - revision: b4fcdfd54f1c5bcd511254b25097518676fdf369
  - direct_write:
    - change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
    - files:
      - approvals.json: changed
    - before_state: verifying
    - after_state: verifying
  - by: unknown agent (hc t14z)
- 2026-11-25T18:10:25Z · direct-native-write · accept → accept · align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct's lifecycle records changed outside Chough: review.json changed, review-attempts.json changed; state stays verifying ^t-5b8e93
  - native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
  - revision: b4fcdfd54f1c5bcd511254b25097518676fdf369
  - direct_write:
    - change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
    - files:
      - review.json: changed
      - review-attempts.json: changed
    - before_state: verifying
    - after_state: verifying
  - by: unknown agent (hc t14z)
- 2026-11-25T18:10:25Z · advance · accept → done · native:archived ^t-2cd4bd
  - native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
  - gate: native:archived
  - revision: b4fcdfd54f1c5bcd511254b25097518676fdf369
  - by: unknown agent (hc t14z)

Closed 2026-11-25: done.

## Receipts

```yaml
receipts: []
```
