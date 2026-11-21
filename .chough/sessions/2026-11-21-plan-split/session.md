---
kind: session
id: 2026-11-21-plan-split
aliases: [plan-split]
objective: Write the agreed plan-stage execution split and test-stage grouping loop into the workflow skill, with tests pinning the wording
intent_scope: ""
modules: []
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: plan-split
pr: ""
criteria: [FLOW-5, FLOW-6, FLOW-8, DRIVE-3, LIFE-10, KNOW-3]
requirements: []
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human:
  - gate: Scope approval of plan-split including src/skill.rs
    holds: plan
    drives: approve
    opened: 2026-11-21T23:24:03Z
run_consents: []
next: Waiting for the widened scope approval of plan-split
---
# Write the agreed plan-stage execution split and test-stage grouping loop into the workflow skill, with tests pinning the wording

## Goal

Write the agreed plan-stage execution split and test-stage grouping loop into the workflow skill, with tests pinning the wording

## Where it stands

- Dogfood of the execution split: one build worker, Flint hc t11r, in this checkout. Not fanned out: the skill text, its installed copy and the pinning tests are one coupled cut, and fewer than 3 similar runs exist, so no speed was invented. Wall-clock from the plan-to-build advance at 2026-11-21T22:06:46Z to skill commit dc416ad3 at 2026-11-21T23:04:59Z was 58 minutes. The changelog newline follow-up 5b43ccf8 landed at 2026-11-21T23:16:02Z. Conflicts: none. Inbox questions: none. Stage-backs: none. Tokens: the host showed a context meter and did not report one task total, so tokens are unknown. Test loop: fmt, the full test lane, lint and skill_releases were green on the first pass. The missing changelog newline was one fix, not a bug. The undeclared src/skill.rs digest registration is one scope gap, not a product defect.

## Decisions and why

- 0xLeif decided "Scope approval of plan-split": "yes I approve" (Grok chat on 2026-11-21, relayed by grok-t11b); recorded by grok-t11b; a relayed claim, not authenticated; ran specsync change approve plan-split as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-21T20:24:14Z · start · intent → intent · plan-split ^t-c8b73d
  - revision: 51b76c626599ce28494a23b1edd740d1dff21854
  - by: unknown agent (hc t11f)
- 2026-11-21T20:33:23Z · advance · intent → design ·  ^t-659104
  - revision: 51b76c626599ce28494a23b1edd740d1dff21854
  - by: unknown agent (hc t11f)
- 2026-11-21T20:33:23Z · skip · design → plan · no visual change ^t-5d8a75
  - revision: 51b76c626599ce28494a23b1edd740d1dff21854
  - by: unknown agent (hc t11f)
- 2026-11-21T20:57:42Z · gate-opened · plan → plan · Scope approval of plan-split ^t-2d2719
  - native_change: plan-split
  - revision: 6e084f477102063c7d32ec79f1f32c91421efc8e
  - holds: plan
  - drives: approve
  - by: unknown agent (hc t11f)
- 2026-11-21T22:05:39Z · direct-native-write · plan → plan · plan-split's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-17ae82
  - native_change: plan-split
  - revision: 7cb20d96c57f55d77c919382d935845cf4569fbe
  - direct_write:
    - change: plan-split
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: unknown agent (hc t11b)
- 2026-11-21T22:05:39Z · gate-closed · plan → plan · Scope approval of plan-split ^t-c7176e
  - native_change: plan-split
  - revision: 7cb20d96c57f55d77c919382d935845cf4569fbe
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes I approve
    - where: Grok chat on 2026-11-21, relayed by grok-t11b
    - recorded_by: grok-t11b
    - summary_digest: 74ae18e0b306c565873f7524687fa6ac87a7ac93c3c6fa739346faeb1274ca19
    - list: 882da74f3137
    - native: specsync change approve plan-split
    - actor: 0xLeif
  - by: unknown agent (hc t11b)
- 2026-11-21T22:06:46Z · advance · plan → build · native:approved ^t-19a9e7
  - native_change: plan-split
  - gate: native:approved
  - revision: 3968a2160e1c39b0dc82433afcf500f02c249a2e
  - by: unknown agent (hc t11f)
- 2026-11-21T23:23:21Z · direct-native-write · build → build · plan-split's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-3d6c63
  - native_change: plan-split
  - revision: c423aa9766eb72c1b8a05e33aeea9eb27793e6bc
  - direct_write:
    - change: plan-split
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: unknown agent (hc t11b)
- 2026-11-21T23:23:21Z · back · build → plan · Leif sent the change back so src/skill.rs can be declared ^t-932ee2
  - native_change: plan-split
  - revision: c423aa9766eb72c1b8a05e33aeea9eb27793e6bc
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition
  - by: unknown agent (hc t11b)
- 2026-11-21T23:24:03Z · gate-opened · plan → plan · Scope approval of plan-split including src/skill.rs ^t-31c6d9
  - native_change: plan-split
  - revision: c423aa9766eb72c1b8a05e33aeea9eb27793e6bc
  - holds: plan
  - drives: approve
  - by: unknown agent (hc t11b)
