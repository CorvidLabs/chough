---
kind: session
id: 2026-11-19-doctor-fix-safety
aliases: [doctor-fix-safety]
objective: "doctor --fix never destroys or hangs: the archive finish keeps evidence its story does not hold (#381) and removes OS metadata (#377), next names the archive finish (#371), doctor names front-matter conflicts and gives no restore advice for a redaction (#374), and the spec walk stays inside the project and is bounded (#402)"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: doctor-fix-safety
pr: ""
criteria: [DRIVE-17]
requirements: [REQ-history-012, REQ-cli-021, REQ-records-001, REQ-records-014]
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
# doctor --fix never destroys or hangs: the archive finish keeps evidence its story does not hold (#381) and removes OS metadata (#377), next names the archive finish (#371), doctor names front-matter conflicts and gives no restore advice for a redaction (#374), and the spec walk stays inside the project and is bounded (#402)

## Goal

doctor --fix never destroys or hangs: the archive finish keeps evidence its story does not hold (#381) and removes OS metadata (#377), next names the archive finish (#371), doctor names front-matter conflicts and gives no restore advice for a redaction (#374), and the spec walk stays inside the project and is bounded (#402)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of doctor-fix-safety": "Approve, with the 3 recommended choices" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, his scope approval of D1 'doctor --fix must not destroy or hang' (#381, #402, #377, #371, #374) for the deltas at bf9f70c, with the three recommended choices: #381 keeps and names the entry, attaching it to the story being later and its own issue; #371 next says commit, then doctor --fix; #374 a Timeline entry changed only by redaction is not reported as rewritten, checked by redacting HEAD's text again); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve doctor-fix-safety as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-19T02:52:53Z · start · intent → intent · doctor --fix never destroys or hangs: the archive finish keeps evidence its story does not hold (#381) and removes OS metadata (#377), next names the archive finish (#371), doctor names front-matter conflicts and gives no restore advice for a redaction (#374), and the spec walk stays inside the project and is bounded (#402) ^t-ff9a59
  - revision: 6936f88f47a2c69f9febf15822678f1d804ff027
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T02:52:53Z · advance · intent → design ·  ^t-83fe0e
  - revision: 6936f88f47a2c69f9febf15822678f1d804ff027
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T02:52:53Z · skip · design → plan · Doctor and next behaviour; nothing visual or interactive changes ^t-9a591f
  - revision: 6936f88f47a2c69f9febf15822678f1d804ff027
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T04:22:54Z · gate-opened · plan → plan · Scope approval of doctor-fix-safety ^t-8ed6a7
  - native_change: doctor-fix-safety
  - revision: bf9f70cdb6e723c070b4cb660d9b43aa4a705a58
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T04:22:54Z · gate-closed · plan → plan · Scope approval of doctor-fix-safety ^t-b60f44
  - native_change: doctor-fix-safety
  - revision: bf9f70cdb6e723c070b4cb660d9b43aa4a705a58
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve, with the 3 recommended choices
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, his scope approval of D1 'doctor --fix must not destroy or hang' (#381, #402, #377, #371, #374) for the deltas at bf9f70c, with the three recommended choices: #381 keeps and names the entry, attaching it to the story being later and its own issue; #371 next says commit, then doctor --fix; #374 a Timeline entry changed only by redaction is not reported as rewritten, checked by redacting HEAD's text again
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve doctor-fix-safety
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T04:22:55Z · advance · plan → build · native:approved ^t-38ef65
  - native_change: doctor-fix-safety
  - gate: native:approved
  - revision: bf9f70cdb6e723c070b4cb660d9b43aa4a705a58
  - by: chough-fixer (claude-opus-5-5, hc t4n)
