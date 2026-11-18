---
kind: session
id: 2026-11-18-fix-339
aliases: [fix-339]
objective: "Conflict-marker follow-ups: markers in fenced code blocks don't count, a stray ======= is caught, case definitions get the same check, and migrate names the 0.5.x source (REQ-records-001, #339)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-339
pr: ""
criteria: [DRIVE-25]
requirements: [REQ-records-001]
cases:
  - TC-records-a-fenced-example-and-a-lone-separator-both-save
  - TC-records-doctor-warns-of-a-lone-conflict-separator
design: []
files: []
observations:
  - 2026-11-18-tc-records-a-fenced-example-and-a-lone-separator-both-save
  - 2026-11-18-tc-records-doctor-warns-of-a-lone-conflict-separator
  - 2026-11-18-tc-records-a-fenced-example-and-a-lone-separator-both-save-ed6ca5
  - 2026-11-18-tc-records-doctor-warns-of-a-lone-conflict-separator-e8c46f
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-339
    holds: accept
    drives: review
    opened: 2026-11-18T21:33:09Z
run_consents: []
next: Continue accept
---
# Conflict-marker follow-ups: markers in fenced code blocks don't count, a stray ======= is caught, case definitions get the same check, and migrate names the 0.5.x source (REQ-records-001, #339)

## Goal

Conflict-marker follow-ups: markers in fenced code blocks don't count, a stray ======= is caught, case definitions get the same check, and migrate names the 0.5.x source (REQ-records-001, #339)

## Where it stands

- Failing tests first: the fenced-example, lone-separator warning, case-definition and migrate-source tests failed on main; markers inside code fences no longer count, a lone ======= is never refused and doctor reports it as an advisory possible_conflict_separator finding, case definitions get conflict_markers findings and a named cases check refusal, and the migration preview and refusal name the 0.5.x source; records, doctor (18), cases and migrate pass

## Decisions and why

- 0xLeif decided "Scope approval of fix-339": "exempt markers inside fenced code blocks, catch a stray =======, case TOML findings, and the migrate message naming the 0.5.x source … Warn only" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: his decisions on the #336 follow-ups filed as CorvidLabs/chough#339, his scope approval for that issue; then for item 2 'Warn only': a lone line of exactly seven '=' with no <<<<<<< or >>>>>>> around it is reported by doctor as 'possible leftover conflict separator at line N', and writes are never refused for it); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-339 as 0xLeif

## What was verified

- [Ran cargo test --locked --test records -- --exact a_fenced_example_and_a_lone_separator_both_save for TC-records-a-fenced-example-and-a-lone-separator-both-save: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-fenced-example-and-a-lone-separator-both-save.md)
- [Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_lone_conflict_separator_without_failing for TC-records-doctor-warns-of-a-lone-conflict-separator: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-doctor-warns-of-a-lone-conflict-separator.md)
- [Ran cargo test --locked --test records -- --exact a_fenced_example_and_a_lone_separator_both_save for TC-records-a-fenced-example-and-a-lone-separator-both-save: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-fenced-example-and-a-lone-separator-both-save-ed6ca5.md)
- [Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_lone_conflict_separator_without_failing for TC-records-doctor-warns-of-a-lone-conflict-separator: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-doctor-warns-of-a-lone-conflict-separator-e8c46f.md)

## Open threads

## Timeline

- 2026-11-18T21:01:27Z · start · intent → intent · Conflict-marker follow-ups: markers in fenced code blocks don't count, a stray ======= is caught, case definitions get the same check, and migrate names the 0.5.x source (REQ-records-001, #339) ^t-132d2d
  - revision: af8ca8901b2a23affcd40ac74865d99637448346
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T21:01:27Z · advance · intent → design ·  ^t-f47670
  - revision: af8ca8901b2a23affcd40ac74865d99637448346
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T21:01:27Z · skip · design → plan · Record checks and messages; nothing visual or interactive changes ^t-34e9bc
  - revision: af8ca8901b2a23affcd40ac74865d99637448346
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T21:31:48Z · gate-opened · plan → plan · Scope approval of fix-339 ^t-6b928f
  - native_change: fix-339
  - revision: af8ca8901b2a23affcd40ac74865d99637448346
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T21:31:48Z · gate-closed · plan → plan · Scope approval of fix-339 ^t-d96c5c
  - native_change: fix-339
  - revision: af8ca8901b2a23affcd40ac74865d99637448346
  - relayed_by:
    - decided_by: 0xLeif
    - words: exempt markers inside fenced code blocks, catch a stray =======, case TOML findings, and the migrate message naming the 0.5.x source … Warn only
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: his decisions on the #336 follow-ups filed as CorvidLabs/chough#339, his scope approval for that issue; then for item 2 'Warn only': a lone line of exactly seven '=' with no <<<<<<< or >>>>>>> around it is reported by doctor as 'possible leftover conflict separator at line N', and writes are never refused for it
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-339
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T21:31:49Z · advance · plan → build · native:approved ^t-a33fba
  - native_change: fix-339
  - gate: native:approved
  - revision: af8ca8901b2a23affcd40ac74865d99637448346
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T21:32:07Z · advance · build → test · native:approved ^t-a28258
  - native_change: fix-339
  - gate: native:approved
  - revision: af8ca8901b2a23affcd40ac74865d99637448346
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T21:33:09Z · advance · test → accept · native:verifying ^t-accba0
  - native_change: fix-339
  - gate: native:verifying
  - revision: af8ca8901b2a23affcd40ac74865d99637448346
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T21:33:09Z · gate-opened · accept → accept · Implementation review of fix-339 ^t-3496cb
  - native_change: fix-339
  - revision: af8ca8901b2a23affcd40ac74865d99637448346
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
