---
kind: session
id: 2026-11-18-fix-268
aliases: [fix-268]
objective: "checkpoint --file and --design values pass the write-time leak guard (REQ-records-006, #268)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-268
pr: ""
criteria: [LIFE-26]
requirements: [REQ-records-006]
cases:
  - TC-records-a-file-or-design-path-passes-the-leak-guard
  - TC-records-an-id-reference-passes-the-leak-guard
  - TC-records-evidence-add-in-a-leaking-session-writes-nothing
design: []
files: []
observations:
  - 2026-11-18-tc-records-a-file-or-design-path-passes-the-leak-guard
  - 2026-11-18-tc-records-a-file-or-design-path-passes-the-leak-guard-3c5e8e
  - 2026-11-18-tc-records-an-id-reference-passes-the-leak-guard
  - 2026-11-18-tc-records-evidence-add-in-a-leaking-session-writes-nothing
  - 2026-11-18-tc-records-a-file-or-design-path-passes-the-leak-guard-5d82c1
  - 2026-11-18-tc-records-an-id-reference-passes-the-leak-guard-b71b57
  - 2026-11-18-tc-records-evidence-add-in-a-leaking-session-writes-nothing-b4d572
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-268
    holds: accept
    drives: review
    opened: 2026-11-18T15:38:53Z
run_consents: []
next: Continue accept
---
# checkpoint --file and --design values pass the write-time leak guard (REQ-records-006, #268)

## Goal

checkpoint --file and --design values pass the write-time leak guard (REQ-records-006, #268)

## Where it stands

- Failing test first: a_token_in_a_file_or_design_path_is_refused_at_the_write failed on main (the --file write was saved); the guard now checks the files and design references and it passes
- Tester findings on #315: ID references were not guarded and evidence add left its entry in a leaking session; both now refuse before writing, red first on the branch, green with the fix; merged main at 66543c2

## Decisions and why

- No requirement text changes: REQ-records-006 already requires every value Chough writes to pass the leak check, so this is --no-spec-change. The guard adds the checkpoint's files and design references; the other references are IDs that validation already limits to an ID form.
- 0xLeif decided "Scope approval of fix-268": "Approve all three" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc t4g), 2026-11-18, in answer to: Approve the scope for all three A-list fixes: #268 (checkpoint --file/--design skip the write-time leak guard), #274 (feature-bug-loop example: no pipefail), #294 (test-only: the gate summary Change line)?); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-268 as 0xLeif

## What was verified

- [Ran cargo test --locked --test leaks -- --exact a_token_in_a_file_or_design_path_is_refused_at_the_write for TC-records-a-file-or-design-path-passes-the-leak-guard: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-file-or-design-path-passes-the-leak-guard.md)
- [Ran cargo test --locked --test leaks -- --exact a_token_in_a_file_or_design_path_is_refused_at_the_write for TC-records-a-file-or-design-path-passes-the-leak-guard: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-file-or-design-path-passes-the-leak-guard-3c5e8e.md)
- [Ran cargo test --locked --test leaks -- --exact a_token_in_an_id_reference_is_refused_even_under_redact for TC-records-an-id-reference-passes-the-leak-guard: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-an-id-reference-passes-the-leak-guard.md)
- [Ran cargo test --locked --test leaks -- --exact evidence_add_in_a_session_holding_a_leak_writes_nothing for TC-records-evidence-add-in-a-leaking-session-writes-nothing: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-evidence-add-in-a-leaking-session-writes-nothing.md)
- [Ran cargo test --locked --test leaks -- --exact a_token_in_a_file_or_design_path_is_refused_at_the_write for TC-records-a-file-or-design-path-passes-the-leak-guard: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-file-or-design-path-passes-the-leak-guard-5d82c1.md)
- [Ran cargo test --locked --test leaks -- --exact a_token_in_an_id_reference_is_refused_even_under_redact for TC-records-an-id-reference-passes-the-leak-guard: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-an-id-reference-passes-the-leak-guard-b71b57.md)
- [Ran cargo test --locked --test leaks -- --exact evidence_add_in_a_session_holding_a_leak_writes_nothing for TC-records-evidence-add-in-a-leaking-session-writes-nothing: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-evidence-add-in-a-leaking-session-writes-nothing-b4d572.md)

## Open threads

## Timeline

- 2026-11-18T15:35:46Z · start · intent → intent · checkpoint --file and --design values pass the write-time leak guard (REQ-records-006, #268) ^t-5ad747
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:36:08Z · advance · intent → design ·  ^t-6b00f3
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:36:08Z · skip · design → plan · Leak-guard coverage of two checkpoint fields; nothing visual or interactive changes ^t-cda388
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:37:01Z · gate-opened · plan → plan · Scope approval of fix-268 ^t-1b87c7
  - native_change: fix-268
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:37:30Z · gate-closed · plan → plan · Scope approval of fix-268 ^t-13d471
  - native_change: fix-268
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve all three
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc t4g), 2026-11-18, in answer to: Approve the scope for all three A-list fixes: #268 (checkpoint --file/--design skip the write-time leak guard), #274 (feature-bug-loop example: no pipefail), #294 (test-only: the gate summary Change line)?
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-268
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:37:35Z · advance · plan → build · native:approved ^t-d72a94
  - native_change: fix-268
  - gate: native:approved
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:38:31Z · advance · build → test · native:approved ^t-af1271
  - native_change: fix-268
  - gate: native:approved
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:38:43Z · advance · test → accept · native:approved ^t-8aba11
  - native_change: fix-268
  - gate: native:approved
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:38:53Z · gate-opened · accept → accept · Implementation review of fix-268 ^t-34ae87
  - native_change: fix-268
  - revision: be6c9f126e448bd33945a52261d55db2a9677d3d
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4g)
