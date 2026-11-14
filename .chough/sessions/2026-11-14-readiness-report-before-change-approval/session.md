---
kind: session
id: 2026-11-14-readiness-report-before-change-approval
aliases: [readiness-report-before-change-approval]
objective: One read-only command lists everything still missing before a change is approved, and names the command that fixes each piece (DRIVE-29).
intent_scope: DRIVE
modules: [cli]
stage: build
stage_status: active
iteration: 1
callers: []
native_change: readiness-report-before-change-approval
pr: ""
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Commit the branch, merge main, then prove the command with local agents before a draft pull request.
---
# One read-only command lists everything still missing before a change is approved, and names the command that fixes each piece (DRIVE-29).

## Goal

One read-only command lists everything still missing before a change is approved, and names the command that fixes each piece (DRIVE-29).

## Where it stands

- chough change ready lists everything still missing before approval and names the command that fixes each piece. fledge fmt, test, lint and the verify lane passed with Fledge 1.8.0, and ready on this change exits 0.

## Decisions and why

- Leif chose Capture as written for DRIVE-29 in chat on 2026-11-14. The sentence is the readiness command before approval.
- 0xLeif decided "Scope approval of readiness-report-before-change-approval": "Approve this scope (Recommended)" (Grok chat, 2026-11-14, choosing the option Approve this scope for this change's scope question. Relayed by grok (hc t1w).); recorded by grok-t1w; a relayed claim, not authenticated; ran specsync change approve readiness-report-before-change-approval as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-14T20:55:49Z · start · intent → intent · One read-only command lists everything still missing before a change is approved, and names the command that fixes each piece (DRIVE-29). ^t-3affaa
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: unknown agent (hc t1w)
- 2026-11-14T20:55:49Z · advance · intent → design ·  ^t-4d51ca
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: unknown agent (hc t1w)
- 2026-11-14T20:55:49Z · skip · design → plan · A CLI report only: no visuals or interactions change. ^t-32b9ae
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: unknown agent (hc t1w)
- 2026-11-14T21:03:18Z · gate-opened · plan → plan · Scope approval of readiness-report-before-change-approval ^t-d0a53e
  - native_change: readiness-report-before-change-approval
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - holds: plan
  - drives: approve
  - by: unknown agent (hc t1w)
- 2026-11-14T21:05:51Z · gate-closed · plan → plan · Scope approval of readiness-report-before-change-approval ^t-871527
  - native_change: readiness-report-before-change-approval
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve this scope (Recommended)
    - where: Grok chat, 2026-11-14, choosing the option Approve this scope for this change's scope question. Relayed by grok (hc t1w).
    - recorded_by: grok-t1w
    - native: specsync change approve readiness-report-before-change-approval
    - actor: 0xLeif
  - by: unknown agent (hc t1w)
- 2026-11-14T21:06:00Z · advance · plan → build · native:approved ^t-bb35c7
  - native_change: readiness-report-before-change-approval
  - gate: native:approved
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: unknown agent (hc t1w)
