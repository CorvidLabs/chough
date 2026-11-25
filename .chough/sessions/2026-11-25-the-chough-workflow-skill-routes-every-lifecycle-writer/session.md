---
kind: session
id: 2026-11-25-the-chough-workflow-skill-routes-every-lifecycle-writer
aliases: [the-chough-workflow-skill-routes-every-lifecycle-writer-through]
objective: "The chough workflow skill routes every lifecycle writer through chough subcommands: direct specsync change invocation becomes a recorded violation with the chough-first order explicit"
intent_scope: process
modules: [workflow]
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
pr: ""
criteria: [DRIVE-20]
requirements: []
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
    holds: accept
    drives: review
    opened: 2026-11-25T16:39:39Z
run_consents: []
next: Continue accept
---
# The chough workflow skill routes every lifecycle writer through chough subcommands: direct specsync change invocation becomes a recorded violation with the chough-first order explicit

## Goal

The chough workflow skill routes every lifecycle writer through chough subcommands: direct specsync change invocation becomes a recorded violation with the chough-first order explicit

## Where it stands

## Decisions and why

- Align the chough-workflow skill with DRIVE-20: the skill routes every lifecycle writer through chough subcommands; direct specsync change invocation is a recorded violation. Reason: the skill's 'Run HI, SpecSync and Fledge yourself' wording invited the PR-38 mistake where the orchestrator drove specsync directly and the chough session trail could not finish done (source: Leif, pi chat t14z, 2026-11-25, choosing 'Fix the skill + rules')
- leif decided "Approve the scope of align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct": "Approve scope (Recommended)" (pi chat t14z interview, 2026-11-25T16:45Z (question: Chough change REQ-cli-072 (skill routes all lifecycle writers through chough): approve scope?)); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change approve align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct as 0xLeif

## What was verified

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
