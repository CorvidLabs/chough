---
kind: session
id: 2026-11-20-bug-add-leaves-the-session
aliases: [bug-add-leaves-the-session]
objective: "bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)"
intent_scope: ""
modules: [cli, history]
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: ""
pr: ""
criteria: [LIFE-5, DRIVE-12]
requirements: [REQ-cli-012, REQ-cli-054, REQ-history-006]
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Read scoped intent and native status, then choose the next action within the authorized scope.
---
# bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)

## Goal

bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)

## Where it stands

## Decisions and why

## What was verified

## Open threads

## Timeline

- 2026-11-20T11:18:44Z · start · intent → intent · bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395) ^t-199ddf
  - revision: 056cbb8029f6535ff74f866a0523893916f682db
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T11:19:10Z · advance · intent → design ·  ^t-e6d040
  - revision: 056cbb8029f6535ff74f866a0523893916f682db
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T11:19:15Z · skip · design → plan · Nothing visual changes: bug add stops writing the session and a bug records the session it was filed in. ^t-a49bfd
  - revision: 056cbb8029f6535ff74f866a0523893916f682db
  - by: chough-fixer (claude-opus-5-5, hc t4n)
