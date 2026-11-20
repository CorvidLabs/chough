---
kind: session
id: 2026-11-20-specsync-6-4
aliases: [specsync-6-4]
objective: "Chough moves to SpecSync 6.4.0: change ready asks SpecSync's approval dry run and stops copying its delta-set rule, spec history is recorded in fragments with no spec merge line, and change ready leaves .specsync/hashes.json alone (#404, #406, #307)"
intent_scope: ""
modules: []
stage: design
stage_status: active
iteration: 1
callers: []
native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
pr: ""
criteria: [DRIVE-29, DRIVE-12]
requirements: []
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Gate \"Scope approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments (native approval)\" closed (approved); run `chough stage advance` to move on."
---
# Chough moves to SpecSync 6.4.0: change ready asks SpecSync's approval dry run and stops copying its delta-set rule, spec history is recorded in fragments with no spec merge line, and change ready leaves .specsync/hashes.json alone (#404, #406, #307)

## Goal

Chough moves to SpecSync 6.4.0: change ready asks SpecSync's approval dry run and stops copying its delta-set rule, spec history is recorded in fragments with no spec merge line, and change ready leaves .specsync/hashes.json alone (#404, #406, #307)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments (native approval)": "Approve (Recommended)" (Leif's own answers in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 18:51Z: to 'Chough #404 (moving Chough onto SpecSync 6.4.0, with the rest of #406 and #307), scope approval. ... Approve the scope?' he answered 'Approve (Recommended)'; to Q1, 'should change ready drop its own delta-set rule, since the dry run names the same missing and extra deltas?', 'A: drop it (Recommended)'; to Q2, 'should the paths check leave out another active change's work on the same branch?', 'A: yes, now (Recommended)'. It merges only after 6.4.0 is on crates.io, ideally with no other Chough PR open. Recorded by t4c itself, at the draft a9b57dd7); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-20T18:27:53Z · start · intent → intent · Chough moves to SpecSync 6.4.0: change ready asks SpecSync's approval dry run and stops copying its delta-set rule, spec history is recorded in fragments with no spec merge line, and change ready leaves .specsync/hashes.json alone (#404, #406, #307) ^t-e4dc64
  - revision: de49485fc5fb93f05bb15db7cd471023beebd8a8
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T18:27:53Z · advance · intent → design ·  ^t-130b88
  - revision: de49485fc5fb93f05bb15db7cd471023beebd8a8
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T18:37:44Z · gate-opened · design → design · Scope approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments (native approval) ^t-da1ade
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: f48c7e80182103df7135024e4f0e7382b97410e7
  - holds: design
  - drives: approve
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T18:52:11Z · gate-closed · design → design · Scope approval of specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments (native approval) ^t-013aac
  - native_change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
  - revision: a9b57dd754f1d6198b28b815bb5d6fd590398901
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve (Recommended)
    - where: Leif's own answers in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 18:51Z: to 'Chough #404 (moving Chough onto SpecSync 6.4.0, with the rest of #406 and #307), scope approval. ... Approve the scope?' he answered 'Approve (Recommended)'; to Q1, 'should change ready drop its own delta-set rule, since the dry run names the same missing and extra deltas?', 'A: drop it (Recommended)'; to Q2, 'should the paths check leave out another active change's work on the same branch?', 'A: yes, now (Recommended)'. It merges only after 6.4.0 is on crates.io, ideally with no other Chough PR open. Recorded by t4c itself, at the draft a9b57dd7
    - recorded_by: t4c
    - native: specsync change approve specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
