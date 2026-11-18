---
kind: session
id: 2026-11-18-fix-286
aliases: [fix-286]
objective: "An archived story's receipts keep each evidence entry's criteria, requirements and limitations (REQ-history-006, #286)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-286
pr: ""
criteria: [KNOW-35]
requirements: [REQ-history-006]
cases: [TC-history-a-receipt-keeps-its-entrys-criteria-requirements-and-limitations]
design: []
files: [src/evidence.rs, src/evidence_migrate.rs, tests/evidence.rs]
observations: [2026-11-18-tc-history-a-receipt-keeps-its-entrys-criteria-requirements]
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-286
    holds: accept
    drives: review
    opened: 2026-11-18T19:58:46Z
run_consents: []
next: Wait for the tester's check and Leif's implementation review of fix-286
---
# An archived story's receipts keep each evidence entry's criteria, requirements and limitations (REQ-history-006, #286)

## Goal

An archived story's receipts keep each evidence entry's criteria, requirements and limitations (REQ-history-006, #286)

## Where it stands

- Receipts keep criteria, requirements and limitations; the receipt test fails on main and passes with the fix

## Decisions and why

- 0xLeif decided "Scope approval of fix-286": "keep all three. Archived receipts keep the entry's criteria, requirements and limitations, so the story says what was verified against what and its honest limits stay visible. This adds the three to REQ-history-006's receipt field list." (Leif's decision comment on GitHub issue #286, 2026-11-18, from his planning interview; his planning session (hc t4c) relayed it to fable-t4b as his scope approval); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-286 as 0xLeif

## What was verified

- [Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-receipt-keeps-its-entrys-criteria-requirements-and-limitations: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-history-a-receipt-keeps-its-entrys-criteria-requirements.md)

## Open threads

## Timeline

- 2026-11-18T19:47:39Z · start · intent → intent · An archived story's receipts keep each evidence entry's criteria, requirements and limitations (REQ-history-006, #286) ^t-e6928d
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:47:39Z · advance · intent → design ·  ^t-cce6ac
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:47:39Z · skip · design → plan · Record content change; nothing visual or interactive changes ^t-ff2e38
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:55:11Z · gate-opened · plan → plan · Scope approval of fix-286 ^t-869162
  - native_change: fix-286
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:55:11Z · gate-closed · plan → plan · Scope approval of fix-286 ^t-1fc3a3
  - native_change: fix-286
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - relayed_by:
    - decided_by: 0xLeif
    - words: keep all three. Archived receipts keep the entry's criteria, requirements and limitations, so the story says what was verified against what and its honest limits stay visible. This adds the three to REQ-history-006's receipt field list.
    - where: Leif's decision comment on GitHub issue #286, 2026-11-18, from his planning interview; his planning session (hc t4c) relayed it to fable-t4b as his scope approval
    - recorded_by: fable-t4b
    - native: specsync change approve fix-286
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:55:13Z · advance · plan → build · native:approved ^t-87842d
  - native_change: fix-286
  - gate: native:approved
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:55:13Z · advance · build → test · native:approved ^t-5fafc7
  - native_change: fix-286
  - gate: native:approved
  - revision: 9dd28de28015ce6f873e237044cc9650e74a3712
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:58:46Z · advance · test → accept · native:verifying ^t-fa36dd
  - native_change: fix-286
  - gate: native:verifying
  - revision: 3978e527802d479546a9a3a057ca24213d0acbfe
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T19:58:46Z · gate-opened · accept → accept · Implementation review of fix-286 ^t-14945e
  - native_change: fix-286
  - revision: 3978e527802d479546a9a3a057ca24213d0acbfe
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
