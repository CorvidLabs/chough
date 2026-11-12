---
kind: session
id: 2026-11-07-explain-how-chough-guides-the-specsync-lifecycle-and-stages-in
aliases: [explain-how-chough-guides-the-specsync-lifecycle-and-stages-in-the-readme-and-walkthrough]
objective: "Document how Chough works: what the agent runs automatically, what each helper command does, and what the human decides, in the README and walkthrough."
intent_scope: FLOW
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
pr: ""
criteria: [FLOW-18, FLOW-13, LIFE-3]
requirements: []
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Read scoped intent and native status, then choose the next action within the authorized scope.
closed: 2026-11-08
outcome: done
receipts: []
---
# Document how Chough works: what the agent runs automatically, what each helper command does, and what the human decides, in the README and walkthrough.

## Goal

Document how Chough works: what the agent runs automatically, what each helper command does, and what the human decides, in the README and walkthrough.

## Where it stands

## Decisions and why

- Reuse existing HI criteria FLOW-18 (docs say what main does), FLOW-13 and LIFE-3 (fresh-agent resume): no new HI or REQ IDs; Leif's request is a documentation clarification of existing behavior, not new product behavior.
- Scope is docs-only: README.md, docs/walkthrough.md, docs/lifecycle.md; docs/cli.md stays the command catalog; the authoritative skill text is already accurate and is not rewritten; no helper, runtime, stage, retention or SpecSync changes.
- Rejected: describing the archive-stage proposal (ch-archive-stage.md) as current behavior; it stays a separate unapproved proposal.
- Leif decided "Scope approval for the documentation change explain-how-chough-guides-the-specsync-lifecycle-and-stages-in-the-readme-and-walkthrough": "approve" (direct Leif reply to orc t19f one-item three-file Chough docs scope question, 2026-11-07; native approval recorded by orc-t19f as 0xLeif at 7184636180a2b866d8da7329eccd5083a253ef29 (hc t1h7, receipt _audits/handoffs/orc-ch-guide-scope-2026-11-07.log)); recorded by k-ch-guide; a relayed claim, not authenticated
- Leif decided "Implementation review of explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18": "yes to all" (direct Leif reply to orc t19f two-item implementation acceptance batch, item 2 Chough docs/PR198, received 2026-11-08 UTC (2026-11-07 America/Denver); native review recorded PASS by orc-t19f as 0xLeif at 83280926f4b864dd8c02a6c0660a9b8a8d3cfd3b (hc t1kc, raw _audits/handoffs/orc-ch-guide-review-2026-11-08.log)); recorded by k-ch-guide; a relayed claim, not authenticated

## What was verified

## Open threads

## Timeline

- 2026-11-07T22:13:01Z · gate-opened · intent → intent · Scope approval for the documentation change explain-how-chough-guides-the-specsync-lifecycle-and-stages-in-the-readme-and-walkthrough ^t-8f52f9
  - native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
  - revision: e2d20dad504ee21fd304150632a21aa71d074d37
- 2026-11-07T23:03:02Z · gate-closed · intent → intent · Scope approval for the documentation change explain-how-chough-guides-the-specsync-lifecycle-and-stages-in-the-readme-and-walkthrough ^t-2834fb
  - native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
  - revision: 7184636180a2b866d8da7329eccd5083a253ef29
  - relayed_by: {"decided_by":"Leif","words":"approve","where":"direct Leif reply to orc t19f one-item three-file Chough docs scope question, 2026-11-07; native approval recorded by orc-t19f as 0xLeif at 7184636180a2b866d8da7329eccd5083a253ef29 (hc t1h7, receipt _audits/handoffs/orc-ch-guide-scope-2026-11-07.log)","recorded_by":"k-ch-guide"}
- 2026-11-07T23:03:17Z · direct-native-write · intent → intent ·  ^t-718a15
  - native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
  - revision: 7184636180a2b866d8da7329eccd5083a253ef29
  - direct_write: {"change":"explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"}],"before_state":"draft","after_state":"approved"}
- 2026-11-07T23:03:17Z · advance · intent → design ·  ^t-32087f
  - native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
  - revision: 7184636180a2b866d8da7329eccd5083a253ef29
- 2026-11-07T23:03:18Z · skip · design → plan · Docs-only change; no visuals or interactions (design applies only to visual changes) ^t-30fb17
  - native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
  - revision: 7184636180a2b866d8da7329eccd5083a253ef29
- 2026-11-07T23:03:18Z · advance · plan → build · native:approved ^t-39b46f
  - native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
  - gate: native:approved
  - revision: 7184636180a2b866d8da7329eccd5083a253ef29
- 2026-11-07T23:14:18Z · advance · build → test · native:approved ^t-216028
  - native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
  - gate: native:approved
  - revision: 3f6bd0d51fa768565691b818f78567d42aca5774
- 2026-11-07T23:27:28Z · advance · test → accept · native:approved ^t-d6ec55
  - native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
  - gate: native:approved
  - revision: 3f6bd0d51fa768565691b818f78567d42aca5774
- 2026-11-07T23:27:28Z · gate-opened · accept → accept · Implementation review of explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18 ^t-00325c
  - native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
  - revision: 3f6bd0d51fa768565691b818f78567d42aca5774
- 2026-11-08T02:05:25Z · gate-closed · accept → accept · Implementation review of explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18 ^t-164ed3
  - native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
  - revision: b8dd7f19eef5e0fc0a224506e60595d7f20a2350
  - relayed_by: {"decided_by":"Leif","words":"yes to all","where":"direct Leif reply to orc t19f two-item implementation acceptance batch, item 2 Chough docs/PR198, received 2026-11-08 UTC (2026-11-07 America/Denver); native review recorded PASS by orc-t19f as 0xLeif at 83280926f4b864dd8c02a6c0660a9b8a8d3cfd3b (hc t1kc, raw _audits/handoffs/orc-ch-guide-review-2026-11-08.log)","recorded_by":"k-ch-guide"}
- 2026-11-08T02:05:46Z · direct-native-write · accept → accept ·  ^t-f9f475
  - native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
  - revision: b8dd7f19eef5e0fc0a224506e60595d7f20a2350
  - direct_write: {"change":"explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18","files":[{"path":"review.json","change":"added"},{"path":"review-attempts.json","change":"added"}],"before_state":"verifying","after_state":"verifying"}
- 2026-11-08T02:05:46Z · advance · accept → done · native:archived ^t-7a3520
  - native_change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
  - gate: native:archived
  - revision: b8dd7f19eef5e0fc0a224506e60595d7f20a2350


Closed 2026-11-08: done.
