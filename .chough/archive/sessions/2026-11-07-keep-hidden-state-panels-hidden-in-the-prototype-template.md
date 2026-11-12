---
kind: session
id: 2026-11-07-keep-hidden-state-panels-hidden-in-the-prototype-template
aliases:
  - keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
objective: Keep hidden state panels hidden in the prototype template whatever display rules a project adds (#58)
intent_scope: FLOW
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
pr: ""
criteria: [FLOW-4, LIFE-9]
requirements: [REQ-cli-019]
cases: [TC-cli-prototype-hidden-panels-stay-hidden, TC-cli-prototype-styles-0-4-0-known-outdated]
design: []
files: []
observations: []
bugs:
  - BUG-prototype-template-shows-every-state-panel-when-a-project-sets-display-073394402cc923c41c0b501c
blockers: []
waiting_on_human: []
run_consents: []
next: "After Leif's review (recorded by orc): chough change ship with the review relay, then audit --strict and pre-push, push, CI, merge pinned"
closed: 2026-11-07
outcome: done
receipts: []
---
# Keep hidden state panels hidden in the prototype template whatever display rules a project adds (#58)

## Goal

Keep hidden state panels hidden in the prototype template whatever display rules a project adds (#58)

## Where it stands

- Fix: [hidden] guard in both styles.css copies and the 0.1.0 to 0.4.0 styles.css digest in KNOWN_PREVIOUS (#58); both tests failed first at 0d0233a0 and pass; cargo test 575 passed

## Decisions and why

- decided: the change is created with specsync change new --no-spec-change directly because chough change new passes no --no-spec-change or --rationale; Chough notes the direct write (REQ-cli-060)
- Leif (0xLeif) decided "Scope approval of keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58": "approved" (orc tab, 2026-11-07, relayed by orc (hc t18) to ch-docs-accuracy as 'Leif approved #58' (orc's wording; Leif's own words were not quoted); orc recorded the native definition approval as 0xLeif at 0d0233a0); recorded by claude (ch-docs-accuracy); a relayed claim, not authenticated
- Leif decided "Implementation review of keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58": "Yes to all 3 (Recommended)" (Moshi question from orc t18, 2026-11-06, reviewing #193 at 95e9aeec); recorded by k-ch-docs; a relayed claim, not authenticated

## What was verified

## Open threads

## Timeline

- 2026-11-07T02:27:05Z · reset · intent → plan · FLOW-4 (approved: one canonical HTML/CSS prototype whose rendered states Leif approves) covers #58, reported by Leif from his Raven session; orc assigned it in the 2026-11-06 burn-down ^t-b595fb
  - native_change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
  - revision: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
  - refresh: plan: native definition; scope or contract changes need native reapproval
- 2026-11-07T02:30:17Z · gate-opened · plan → plan · Scope approval of keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58 ^t-406b40
  - native_change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
  - revision: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
- 2026-11-07T02:31:50Z · gate-closed · plan → plan · Scope approval of keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58 ^t-621fa7
  - native_change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
  - revision: 0d0233a082a9b181ac8aee5f44f2cac7a85df8d5
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"approved","where":"orc tab, 2026-11-07, relayed by orc (hc t18) to ch-docs-accuracy as 'Leif approved #58' (orc's wording; Leif's own words were not quoted); orc recorded the native definition approval as 0xLeif at 0d0233a0","recorded_by":"claude (ch-docs-accuracy)"}
- 2026-11-07T02:31:52Z · direct-native-write · plan → plan ·  ^t-cc40c7
  - native_change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
  - revision: 0d0233a082a9b181ac8aee5f44f2cac7a85df8d5
  - direct_write: {"change":"keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"}],"before_state":"draft","after_state":"approved"}
- 2026-11-07T02:31:52Z · advance · plan → build · native:approved ^t-ab114c
  - native_change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
  - gate: native:approved
  - revision: 0d0233a082a9b181ac8aee5f44f2cac7a85df8d5
- 2026-11-07T02:44:10Z · advance · build → test · native:approved ^t-13235c
  - native_change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
  - gate: native:approved
  - revision: 24532be1135a3b4392e52e5b107a9b52be64ec28
- 2026-11-07T02:44:14Z · advance · test → accept · native:approved ^t-0ea452
  - native_change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
  - gate: native:approved
  - revision: 24532be1135a3b4392e52e5b107a9b52be64ec28
- 2026-11-07T02:44:27Z · gate-opened · accept → accept · Implementation review of keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58 ^t-3c0a01
  - native_change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
  - revision: 24532be1135a3b4392e52e5b107a9b52be64ec28
- 2026-11-07T03:45:14Z · gate-closed · accept → accept · Implementation review of keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58 ^t-189d3b
  - native_change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
  - revision: 74b77d1dda1b54e4bb06f911b1713e960322ddfb
  - relayed_by: {"decided_by":"Leif","words":"Yes to all 3 (Recommended)","where":"Moshi question from orc t18, 2026-11-06, reviewing #193 at 95e9aeec","recorded_by":"k-ch-docs"}
- 2026-11-07T04:50:06Z · direct-native-write · accept → accept ·  ^t-2506de
  - native_change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
  - revision: 74b77d1dda1b54e4bb06f911b1713e960322ddfb
  - direct_write: {"change":"keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58","files":[{"path":"review.json","change":"added"},{"path":"review-attempts.json","change":"added"}],"before_state":"verifying","after_state":"verifying"}
- 2026-11-07T04:50:06Z · advance · accept → done · Finish the done move for #58 (PR #193) after k-ch-docs's ship; assigned by orc t18; glm-finish ^t-d5951d
  - native_change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
  - gate: native:archived
  - revision: 74b77d1dda1b54e4bb06f911b1713e960322ddfb


Closed 2026-11-07: done.
