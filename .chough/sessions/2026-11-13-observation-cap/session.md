---
kind: session
id: 2026-11-13-observation-cap
aliases: [observation-cap]
objective: "0.6.1: at the 400-observation cap, every write that adds an observation refuses before writing anything and names chough session shelve"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
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
next: Continue build
---
# 0.6.1: at the 400-observation cap, every write that adds an observation refuses before writing anything and names chough session shelve

## Goal

0.6.1: at the 400-observation cap, every write that adds an observation refuses before writing anything and names chough session shelve

## Where it stands

## Decisions and why

- Leif decided "Approve the intent: at the observation cap, writes refuse before writing and name the recovery": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Observation cap: cases run and evidence add refuse at 400 before writing anything, and name chough session shelve; next stops counting unlinked entries as a case's latest run'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Observation cap: cases run and evidence add refuse at 400 before writing anything, and name chough session shelve; next stops counting unlinked entries as a case's latest run'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-13T02:08:33Z · start · intent → intent · 0.6.1: at the 400-observation cap, every write that adds an observation refuses before writing anything and names chough session shelve ^t-e5d776
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:08:33Z · gate-opened · intent → intent · Approve the intent: at the observation cap, writes refuse before writing and name the recovery ^t-e293d7
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: intent
  - drives: none
  - by: claude-code (hc t382)
- 2026-11-13T02:08:33Z · gate-closed · intent → intent · Approve the intent: at the observation cap, writes refuse before writing and name the recovery ^t-6752fa
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Observation cap: cases run and evidence add refuse at 400 before writing anything, and name chough session shelve; next stops counting unlinked entries as a case's latest run'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
  - by: claude-code (hc t382)
- 2026-11-13T02:08:33Z · advance · intent → design · Leif approved 0.6.1's PRs (02:03Z, relayed by t370) ^t-fc8207
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:08:33Z · skip · design → plan · No visuals or interactions change ^t-64f1ed
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:09:08Z · gate-opened · plan → plan · Approve the scope of 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough ^t-9b7c49
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T02:09:08Z · gate-closed · plan → plan · Approve the scope of 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough ^t-f963d4
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Observation cap: cases run and evidence add refuse at 400 before writing anything, and name chough session shelve; next stops counting unlinked entries as a case's latest run'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T02:09:10Z · advance · plan → build · Leif approved the scope (02:03Z, relayed by t370) ^t-7ddb33
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - gate: native:approved
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
