---
kind: session
id: 2026-11-20-g2-gate-summary
aliases: [g2-gate-summary]
objective: "The gate summary counts evidence without a case, shows uncommitted work in its diff and a base for a session started before the first commit; doctor names the journal post to commit; without SpecSync, --to done across plan or accept says no human decided and needs --reason (#389, #390)"
intent_scope: ""
modules: []
stage: intent
stage_status: active
iteration: 1
callers: []
native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
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
next: "Gate \"Scope approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason (native approval)\" closed (approved); run `chough stage advance` to move on."
---
# The gate summary counts evidence without a case, shows uncommitted work in its diff and a base for a session started before the first commit; doctor names the journal post to commit; without SpecSync, --to done across plan or accept says no human decided and needs --reason (#389, #390)

## Goal

The gate summary counts evidence without a case, shows uncommitted work in its diff and a base for a session started before the first commit; doctor names the journal post to commit; without SpecSync, --to done across plan or accept says no human decided and needs --reason (#389, #390)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason (native approval)": "Approve (Recommended)" (Leif's own answer in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 16:04Z, to: 'Chough G2 (#389's gate summary and #390), scope approval. Four truth-telling fixes found on main ... It changes some of your approved words (REQ-cli-061's cases and its diff range) and adds that one refusal, so it's not a standing-OK case. Approve the scope as drafted?' Recorded by t4c itself, at the draft 17e01d68); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-20T16:02:17Z · start · intent → intent · The gate summary counts evidence without a case, shows uncommitted work in its diff and a base for a session started before the first commit; doctor names the journal post to commit; without SpecSync, --to done across plan or accept says no human decided and needs --reason (#389, #390) ^t-f64d53
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T16:04:34Z · gate-opened · intent → intent · Scope approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason (native approval) ^t-7422bc
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 17e01d681dd0a57fbc968d589825df64b52dcc7c
  - holds: intent
  - drives: approve
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T16:04:34Z · gate-closed · intent → intent · Scope approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason (native approval) ^t-a7d620
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 17e01d681dd0a57fbc968d589825df64b52dcc7c
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve (Recommended)
    - where: Leif's own answer in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 16:04Z, to: 'Chough G2 (#389's gate summary and #390), scope approval. Four truth-telling fixes found on main ... It changes some of your approved words (REQ-cli-061's cases and its diff range) and adds that one refusal, so it's not a standing-OK case. Approve the scope as drafted?' Recorded by t4c itself, at the draft 17e01d68
    - recorded_by: t4c
    - native: specsync change approve fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
