---
kind: session
id: 2026-11-13-fix-245-index-ignored
aliases: [fix-245-index-ignored]
objective: "Fix #245: chough library index links no source Git ignores, so a fresh clone's index check passes"
intent_scope: ""
modules: []
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: ""
pr: ""
criteria: []
requirements: []
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue plan
---
# Fix #245: chough library index links no source Git ignores, so a fresh clone's index check passes

## Goal

Fix #245: chough library index links no source Git ignores, so a fresh clone's index check passes

## Where it stands

## Decisions and why

- Leif decided "Approve fixing #245: chough library index stops linking skill copies that Git ignores": "Approve (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T14:12Z, answering: 'Chough #245 (from corvid-agent): chough library index stops linking skill copies that Git ignores, so the index check passes in a fresh clone and in CI, where it fails today. Small wording change, with a failing test ready. Approve?' (option text: 't37f builds it. A separate warning in chough init would come later as its own change.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated

## What was verified

## Open threads

## Timeline

- 2026-11-13T14:13:02Z · start · intent → intent · Fix #245: chough library index links no source Git ignores, so a fresh clone's index check passes ^t-643827
  - revision: d2d2f60d5be07b6c43a827f576a3b974578af8b7
  - by: claude-code (hc t382)
- 2026-11-13T14:13:02Z · gate-opened · intent → intent · Approve fixing #245: chough library index stops linking skill copies that Git ignores ^t-c4cdfb
  - revision: d2d2f60d5be07b6c43a827f576a3b974578af8b7
  - holds: intent
  - drives: none
  - by: claude-code (hc t382)
- 2026-11-13T14:13:02Z · gate-closed · intent → intent · Approve fixing #245: chough library index stops linking skill copies that Git ignores ^t-797fe6
  - revision: d2d2f60d5be07b6c43a827f576a3b974578af8b7
  - relayed_by:
    - decided_by: Leif
    - words: Approve (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T14:12Z, answering: 'Chough #245 (from corvid-agent): chough library index stops linking skill copies that Git ignores, so the index check passes in a fresh clone and in CI, where it fails today. Small wording change, with a failing test ready. Approve?' (option text: 't37f builds it. A separate warning in chough init would come later as its own change.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
  - by: claude-code (hc t382)
- 2026-11-13T14:13:02Z · advance · intent → design · Leif approved the intent (14:12Z, relayed by t370) ^t-780060
  - revision: d2d2f60d5be07b6c43a827f576a3b974578af8b7
  - by: claude-code (hc t382)
- 2026-11-13T14:13:02Z · skip · design → plan · No visuals or interactions change ^t-8f1126
  - revision: d2d2f60d5be07b6c43a827f576a3b974578af8b7
  - by: claude-code (hc t382)
