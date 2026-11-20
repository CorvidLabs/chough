---
kind: session
id: 2026-11-20-260-whole-or-nothing
aliases: [260-whole-or-nothing]
objective: "A new record is written whole or not at all, so a full disk or a file-size limit never leaves a 0-byte or cut-off record at its name, and an empty record is named as such (#260, #352)"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
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
# A new record is written whole or not at all, so a full disk or a file-size limit never leaves a 0-byte or cut-off record at its name, and an empty record is named as such (#260, #352)

## Goal

A new record is written whole or not at all, so a full disk or a file-size limit never leaves a 0-byte or cut-off record at its name, and an empty record is named as such (#260, #352)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-260-a-new-record-is-written-whole-or-not-at-all (native approval)": "Standing OK, tell me after" (Leif's standing OK for scope approvals (2026-11-20 dialog, relayed via t4c): his answer 'Standing OK, tell me after' in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 04:17Z. Confirmed by t4c for this change on fable-lead's summary (one criterion added to each changed requirement, nothing removed, no refusal). Recorded by t4c itself, at the draft b7550830); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve fix-260-a-new-record-is-written-whole-or-not-at-all as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-20T11:02:21Z · start · intent → intent · A new record is written whole or not at all, so a full disk or a file-size limit never leaves a 0-byte or cut-off record at its name, and an empty record is named as such (#260, #352) ^t-e8a776
  - revision: 056cbb8029f6535ff74f866a0523893916f682db
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T14:13:59Z · gate-opened · intent → intent · Scope approval of fix-260-a-new-record-is-written-whole-or-not-at-all (native approval) ^t-1124df
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: b75508306c8b9073e75eb0f79de5b10d3d09e84d
  - holds: intent
  - drives: approve
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T14:13:59Z · gate-closed · intent → intent · Scope approval of fix-260-a-new-record-is-written-whole-or-not-at-all (native approval) ^t-24f961
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: b75508306c8b9073e75eb0f79de5b10d3d09e84d
  - relayed_by:
    - decided_by: 0xLeif
    - words: Standing OK, tell me after
    - where: Leif's standing OK for scope approvals (2026-11-20 dialog, relayed via t4c): his answer 'Standing OK, tell me after' in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 04:17Z. Confirmed by t4c for this change on fable-lead's summary (one criterion added to each changed requirement, nothing removed, no refusal). Recorded by t4c itself, at the draft b7550830
    - recorded_by: t4c
    - native: specsync change approve fix-260-a-new-record-is-written-whole-or-not-at-all
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T16:22:24Z · advance · intent → design ·  ^t-bd7097
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: ca3b03608717301c055316009a047504fb271bc4
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T16:22:24Z · advance · design → plan ·  ^t-0664ef
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - revision: ca3b03608717301c055316009a047504fb271bc4
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T16:22:25Z · advance · plan → build · native:approved ^t-3b6ab7
  - native_change: fix-260-a-new-record-is-written-whole-or-not-at-all
  - gate: native:approved
  - revision: ca3b03608717301c055316009a047504fb271bc4
  - by: fable (claude-opus-5-5, hc t85)
