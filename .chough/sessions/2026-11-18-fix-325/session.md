---
kind: session
id: 2026-11-18-fix-325
aliases: [fix-325]
objective: "A refused:checkpoint-busy evidence add leaves nothing behind, so a rerun does not duplicate the entry (REQ-cli-040, #325)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-325
pr: ""
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
cases: [TC-cli-a-busy-evidence-add-leaves-nothing]
design: []
files: []
observations:
  - 2026-11-18-tc-cli-a-busy-evidence-add-leaves-nothing
  - 2026-11-18-tc-cli-a-busy-evidence-add-leaves-nothing-583391
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-325
    holds: accept
    drives: review
    opened: 2026-11-18T18:37:46Z
run_consents: []
next: Continue accept
---
# A refused:checkpoint-busy evidence add leaves nothing behind, so a rerun does not duplicate the entry (REQ-cli-040, #325)

## Goal

A refused:checkpoint-busy evidence add leaves nothing behind, so a rerun does not duplicate the entry (REQ-cli-040, #325)

## Where it stands

- Failing test first: a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved failed on main (the entry was written before the lock); evidence::record now takes the writer lock first, the test passes, and #317's lock test waits for the block instead of the entry

## Decisions and why

- 0xLeif decided "Scope approval of fix-325": "approved as a fix. A refused:checkpoint-busy evidence add leaves nothing behind, so a rerun doesn't duplicate the entry." (Leif's decision comment on CorvidLabs/chough#325 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval for this change); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-325 as 0xLeif

## What was verified

- [Ran cargo test --locked --test evidence -- --exact a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved for TC-cli-a-busy-evidence-add-leaves-nothing: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-a-busy-evidence-add-leaves-nothing.md)
- [Ran cargo test --locked --test evidence -- --exact a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved for TC-cli-a-busy-evidence-add-leaves-nothing: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-a-busy-evidence-add-leaves-nothing-583391.md)

## Open threads

## Timeline

- 2026-11-18T18:34:19Z · start · intent → intent · A refused:checkpoint-busy evidence add leaves nothing behind, so a rerun does not duplicate the entry (REQ-cli-040, #325) ^t-29a60b
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:34:19Z · advance · intent → design ·  ^t-6f85de
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:34:19Z · skip · design → plan · Writer-lock ordering in the evidence save; nothing visual or interactive changes ^t-729589
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:34:41Z · gate-opened · plan → plan · Scope approval of fix-325 ^t-f7e70b
  - native_change: fix-325
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:34:41Z · gate-closed · plan → plan · Scope approval of fix-325 ^t-f56f63
  - native_change: fix-325
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - relayed_by:
    - decided_by: 0xLeif
    - words: approved as a fix. A refused:checkpoint-busy evidence add leaves nothing behind, so a rerun doesn't duplicate the entry.
    - where: Leif's decision comment on CorvidLabs/chough#325 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval for this change
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-325
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:34:43Z · advance · plan → build · native:approved ^t-8d620d
  - native_change: fix-325
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:37:13Z · advance · build → test · native:approved ^t-b62fe2
  - native_change: fix-325
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:37:46Z · advance · test → accept · native:approved ^t-bc89ac
  - native_change: fix-325
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:37:46Z · gate-opened · accept → accept · Implementation review of fix-325 ^t-dff9a2
  - native_change: fix-325
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
