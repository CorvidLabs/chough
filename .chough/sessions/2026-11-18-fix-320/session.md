---
kind: session
id: 2026-11-18-fix-320
aliases: [fix-320]
objective: "A leaking ID reference's refusal names the recovery, and doctor --fix --redact never rewrites an ID (REQ-history-012, REQ-cli-055, #320)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-320
pr: ""
criteria: [LIFE-26]
requirements: [REQ-history-012, REQ-cli-055]
cases: [TC-cli-doctor-redact-never-rewrites-an-id]
design: []
files: []
observations:
  - 2026-11-18-tc-cli-doctor-redact-never-rewrites-an-id
  - 2026-11-18-tc-cli-doctor-redact-never-rewrites-an-id-47c83d
  - 2026-11-18-tc-cli-doctor-redact-never-rewrites-an-id-0155b6
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-320
    holds: accept
    drives: review
    opened: 2026-11-18T19:03:38Z
run_consents: []
next: Continue accept
---
# A leaking ID reference's refusal names the recovery, and doctor --fix --redact never rewrites an ID (REQ-history-012, REQ-cli-055, #320)

## Goal

A leaking ID reference's refusal names the recovery, and doctor --fix --redact never rewrites an ID (REQ-history-012, REQ-cli-055, #320)

## Where it stands

- Failing tests first: a_leaking_id_reference_names_the_recovery and doctor_redact_never_rewrites_an_id failed on main (no recovery named; the doctor rewrote the ID); the refusal now names chough session shelve <id>, and the doctor's leak fix leaves ID lines byte for byte with their findings skipped, so all 11 leak tests pass
- #335 tester BLOCK fixed: modules and regression_baseline are ID keys, so doctor --fix --redact leaves those lines byte for byte with their findings skipped; doctor_redact_never_rewrites_modules_or_a_regression_baseline failed without the two keys and passes with them; the docs row notes that a body link to a file whose name holds a match is redacted with the body

## Decisions and why

- The refusal's recovery fits REQ-records-006 as it stands; the doctor's never rewriting an ID changes REQ-history-012's leak fix and REQ-cli-055's --fix, since the doctor code is cli's. session new --module's leak is left on #320, outside Leif's decision.
- 0xLeif decided "Scope approval of fix-320": "approved as a fix. The refusal names the recovery (chough session shelve <id>, or a command that removes the reference), and doctor --fix --redact never rewrites an ID." (Leif's decision comment on CorvidLabs/chough#320 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval for this change); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-320 as 0xLeif

## What was verified

- [Ran cargo test --locked --test leaks -- --exact doctor_redact_never_rewrites_an_id for TC-cli-doctor-redact-never-rewrites-an-id: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-doctor-redact-never-rewrites-an-id.md)
- [Ran cargo test --locked --test leaks -- --exact doctor_redact_never_rewrites_an_id for TC-cli-doctor-redact-never-rewrites-an-id: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-doctor-redact-never-rewrites-an-id-47c83d.md)
- [Ran cargo test --locked --test leaks -- --exact doctor_redact_never_rewrites_an_id for TC-cli-doctor-redact-never-rewrites-an-id: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-doctor-redact-never-rewrites-an-id-0155b6.md)

## Open threads

## Timeline

- 2026-11-18T18:39:30Z · start · intent → intent · A leaking ID reference's refusal names the recovery, and doctor --fix --redact never rewrites an ID (REQ-history-012, REQ-cli-055, #320) ^t-0d1689
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:39:30Z · advance · intent → design ·  ^t-b6acf1
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:39:30Z · skip · design → plan · Leak refusal text and the doctor's redaction; nothing visual or interactive changes ^t-3fc569
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:40:11Z · gate-opened · plan → plan · Scope approval of fix-320 ^t-690134
  - native_change: fix-320
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:40:11Z · gate-closed · plan → plan · Scope approval of fix-320 ^t-2933b7
  - native_change: fix-320
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - relayed_by:
    - decided_by: 0xLeif
    - words: approved as a fix. The refusal names the recovery (chough session shelve <id>, or a command that removes the reference), and doctor --fix --redact never rewrites an ID.
    - where: Leif's decision comment on CorvidLabs/chough#320 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval for this change
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-320
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:40:12Z · advance · plan → build · native:approved ^t-ba4e50
  - native_change: fix-320
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:03:33Z · advance · build → test · native:approved ^t-b1ece8
  - native_change: fix-320
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:03:38Z · advance · test → accept · native:verifying ^t-45914b
  - native_change: fix-320
  - gate: native:verifying
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:03:38Z · gate-opened · accept → accept · Implementation review of fix-320 ^t-5e79c5
  - native_change: fix-320
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
