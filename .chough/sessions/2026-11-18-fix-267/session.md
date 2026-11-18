---
kind: session
id: 2026-11-18-fix-267
aliases: [fix-267]
objective: "A credential glued to a hyphen, as in a slug or run name, is refused as a leak like any other (REQ-records-006, #267)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-267
pr: ""
criteria: [LIFE-26]
requirements: [REQ-records-006]
cases: [TC-records-a-hyphen-glued-token-is-a-leak]
design: []
files: [src/leaks.rs, tests/leaks.rs]
observations:
  - 2026-11-18-tc-records-a-hyphen-glued-token-is-a-leak
  - 2026-11-18-tc-records-a-hyphen-glued-token-is-a-leak-aa5529
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-267
    holds: accept
    drives: review
    opened: 2026-11-18T00:36:59Z
run_consents: []
next: "Wait for Leif's implementation review of fix-267 on PR #303; do not ship"
---
# A credential glued to a hyphen, as in a slug or run name, is refused as a leak like any other (REQ-records-006, #267)

## Goal

A credential glued to a hyphen, as in a slug or run name, is refused as a leak like any other (REQ-records-006, #267)

## Where it stands

- Failing tests first: leaks::tests::a_token_glued_to_a_hyphen_is_found and a_hyphen_glued_token_is_refused_at_the_write_and_found_by_the_doctor failed with main's boundary rule
- A hyphen before a token prefix is a boundary; both tests pass

## Decisions and why

- No requirement text changes: REQ-records-006 already refuses a credential token form in every value, so this is --no-spec-change. A hyphen before a token prefix becomes a boundary in both matcher call sites (the prefixed forms and the AWS key form); a letter, digit or underscore still continues a word. #268 is parked and untouched.
- Leif, 2026-11-17, in the task he gave Fable: 'implement only defects the existing contract already requires, open draft pull requests, and stop at every human gate.' The code and tests were written on that instruction before scope approval; Fable approves nothing.
- 0xLeif decided "Scope approval of fix-267": "record my scope approval of fix-272, fix-267, fix-262 and fix-260 as 0xLeif" (Claude Code session with Fable, 2026-11-17, after his 'i approve all' to Fable's report of the four scope gates); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-267 as 0xLeif

## What was verified

- [Ran cargo test --locked --test leaks -- --exact a_hyphen_glued_token_is_refused_at_the_write_and_found_by_the_doctor for TC-records-a-hyphen-glued-token-is-a-leak: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-hyphen-glued-token-is-a-leak.md)
- [Ran cargo test --locked --test leaks -- --exact a_hyphen_glued_token_is_refused_at_the_write_and_found_by_the_doctor for TC-records-a-hyphen-glued-token-is-a-leak: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-hyphen-glued-token-is-a-leak-aa5529.md)

## Open threads

## Timeline

- 2026-11-18T00:05:45Z · start · intent → intent · A credential glued to a hyphen, as in a slug or run name, is refused as a leak like any other (REQ-records-006, #267) ^t-5b9fdb
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:05:56Z · advance · intent → design ·  ^t-de5983
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:05:56Z · skip · design → plan · Leak-guard matching change; nothing visual or interactive changes ^t-c54a39
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:13:00Z · gate-opened · plan → plan · Scope approval of fix-267 ^t-865808
  - native_change: fix-267
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:35:48Z · gate-closed · plan → plan · Scope approval of fix-267 ^t-7d4bed
  - native_change: fix-267
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - relayed_by:
    - decided_by: 0xLeif
    - words: record my scope approval of fix-272, fix-267, fix-262 and fix-260 as 0xLeif
    - where: Claude Code session with Fable, 2026-11-17, after his 'i approve all' to Fable's report of the four scope gates
    - recorded_by: fable-t4b
    - native: specsync change approve fix-267
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:36:56Z · advance · plan → build · native:approved ^t-990fe5
  - native_change: fix-267
  - gate: native:approved
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:36:56Z · advance · build → test · native:approved ^t-af7c8a
  - native_change: fix-267
  - gate: native:approved
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:36:56Z · advance · test → accept · native:approved ^t-e79218
  - native_change: fix-267
  - gate: native:approved
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T00:36:59Z · gate-opened · accept → accept · Implementation review of fix-267 ^t-b789b9
  - native_change: fix-267
  - revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
