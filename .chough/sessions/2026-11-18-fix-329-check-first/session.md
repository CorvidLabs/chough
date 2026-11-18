---
kind: session
id: 2026-11-18-fix-329-check-first
aliases: [fix-329-check-first]
objective: bug add checks that the active session can take another bug link before writing the bug, and refuses with nothing written (#343 review P2)
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-329-check-first
pr: ""
criteria: [LIFE-5]
requirements: [REQ-cli-012]
cases: []
design: []
files: []
observations:
  - 2026-11-18-tc-cli-bug-add-refuses-before-writing-when-the-session-is-full
  - 2026-11-18-tc-cli-bug-add-joins-the-active-session-6ae083
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-329-check-first
    holds: accept
    drives: review
    opened: 2026-11-18T23:21:05Z
run_consents: []
next: Continue accept
---
# bug add checks that the active session can take another bug link before writing the bug, and refuses with nothing written (#343 review P2)

## Goal

bug add checks that the active session can take another bug link before writing the bug, and refuses with nothing written (#343 review P2)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-329-check-first": "Yes, for today" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: his standing rule for today, that a real gap his review thread finds in an already-approved PR is fixed inside its approved scope as a small follow-up change, applied by hc t4c to his P2 thread on #343 ('Preflight the checkpoint bound before creating the bug'), with t4c's design point: check that the session can take another bug link before writing anything, and refuse with nothing written); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-329-check-first as 0xLeif

## What was verified

- [Ran cargo test --locked --test bugs -- --exact bug_add_refuses_before_writing_when_the_session_is_full for TC-cli-bug-add-refuses-before-writing-when-the-session-is-full: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-bug-add-refuses-before-writing-when-the-session-is-full.md)
- [Ran cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it for TC-cli-bug-add-joins-the-active-session: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-bug-add-joins-the-active-session-6ae083.md)

## Open threads

## Timeline

- 2026-11-18T23:14:08Z · start · intent → intent · bug add checks that the active session can take another bug link before writing the bug, and refuses with nothing written (#343 review P2) ^t-225540
  - revision: 2e371c8855e4ea8ca965634b59c56c26b0329d06
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:14:18Z · advance · intent → design ·  ^t-3494cc
  - revision: 2e371c8855e4ea8ca965634b59c56c26b0329d06
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:14:22Z · skip · design → plan · A check before a bug write; nothing visual or interactive changes ^t-1d345e
  - revision: 2e371c8855e4ea8ca965634b59c56c26b0329d06
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:15:43Z · gate-opened · plan → plan · Scope approval of fix-329-check-first ^t-fab830
  - native_change: fix-329-check-first
  - revision: 2e371c8855e4ea8ca965634b59c56c26b0329d06
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:15:43Z · gate-closed · plan → plan · Scope approval of fix-329-check-first ^t-2d411b
  - native_change: fix-329-check-first
  - revision: 2e371c8855e4ea8ca965634b59c56c26b0329d06
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, for today
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: his standing rule for today, that a real gap his review thread finds in an already-approved PR is fixed inside its approved scope as a small follow-up change, applied by hc t4c to his P2 thread on #343 ('Preflight the checkpoint bound before creating the bug'), with t4c's design point: check that the session can take another bug link before writing anything, and refuse with nothing written
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-329-check-first
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:15:49Z · advance · plan → build · native:approved ^t-a9e161
  - native_change: fix-329-check-first
  - gate: native:approved
  - revision: 2e371c8855e4ea8ca965634b59c56c26b0329d06
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:20:54Z · advance · build → test · native:verifying ^t-d362a5
  - native_change: fix-329-check-first
  - gate: native:verifying
  - revision: 5bf4ac67509f06b528373db43a61be9137bcf711
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:20:56Z · advance · test → accept · native:verifying ^t-d92e2d
  - native_change: fix-329-check-first
  - gate: native:verifying
  - revision: 5bf4ac67509f06b528373db43a61be9137bcf711
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:21:05Z · gate-opened · accept → accept · Implementation review of fix-329-check-first ^t-ebb0c1
  - native_change: fix-329-check-first
  - revision: 5bf4ac67509f06b528373db43a61be9137bcf711
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
