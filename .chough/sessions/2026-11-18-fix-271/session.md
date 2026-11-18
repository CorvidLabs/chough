---
kind: session
id: 2026-11-18-fix-271
aliases: [fix-271]
objective: "Each shell pins its session with --session or CHOUGH_SESSION, whatever the shared pointer says (REQ-cli-037, #271)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-271
pr: ""
criteria: [DRIVE-3]
requirements: [REQ-cli-037]
cases: [TC-cli-a-pinned-shell-writes-its-own-session]
design: []
files: []
observations: [2026-11-18-tc-cli-a-pinned-shell-writes-its-own-session]
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-271
    holds: accept
    drives: review
    opened: 2026-11-18T18:31:58Z
run_consents: []
next: Continue accept
---
# Each shell pins its session with --session or CHOUGH_SESSION, whatever the shared pointer says (REQ-cli-037, #271)

## Goal

Each shell pins its session with --session or CHOUGH_SESSION, whatever the shared pointer says (REQ-cli-037, #271)

## Where it stands

- Failing tests first: a_pinned_shell_writes_its_own_session_whatever_the_pointer_says and a_pin_to_a_session_that_is_not_open_refuses_with_nothing_written failed on main (no --session option); the global --session and CHOUGH_SESSION now pin the command's session in main without moving the pointer, init and inbox add keep their own --session, and all three tests pass; the five hc templates tell agents to export CHOUGH_SESSION, with new versions in templates.toml

## Decisions and why

- The per-shell option is a global --session, which clap leaves out of init and inbox add because they define their own --session, so both keep their meanings; CHOUGH_SESSION is read once in main beside it, and session new and session use ignore it. The pin resolves in sessions::active and never moves the pointer.
- 0xLeif decided "Scope approval of fix-271": "Approve" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: his decision comment on CorvidLabs/chough#271 ('pin per shell': CHOUGH_SESSION plus --session on every write command, hc launches set the variable), then 'Approve' for the REQ-cli-037 wording shown to him in plain words, and 'Yes, same PR' for updating the five versioned hc templates in integrations/ to export CHOUGH_SESSION); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-271 as 0xLeif

## What was verified

- [Ran cargo test --locked --test session_pin -- --exact a_pinned_shell_writes_its_own_session_whatever_the_pointer_says for TC-cli-a-pinned-shell-writes-its-own-session: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-a-pinned-shell-writes-its-own-session.md)

## Open threads

## Timeline

- 2026-11-18T18:22:58Z · start · intent → intent · Each shell pins its session with --session or CHOUGH_SESSION, whatever the shared pointer says (REQ-cli-037, #271) ^t-0f4427
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:22:58Z · advance · intent → design ·  ^t-2da1e2
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:22:58Z · skip · design → plan · A session option and environment variable; nothing visual or interactive changes ^t-c8c075
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:24:04Z · gate-opened · plan → plan · Scope approval of fix-271 ^t-695750
  - native_change: fix-271
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:25:48Z · direct-native-write · plan → plan · fix-271's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-16734d
  - native_change: fix-271
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - direct_write:
    - change: fix-271
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:25:48Z · gate-closed · plan → plan · Scope approval of fix-271 ^t-1bf1d7
  - native_change: fix-271
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: his decision comment on CorvidLabs/chough#271 ('pin per shell': CHOUGH_SESSION plus --session on every write command, hc launches set the variable), then 'Approve' for the REQ-cli-037 wording shown to him in plain words, and 'Yes, same PR' for updating the five versioned hc templates in integrations/ to export CHOUGH_SESSION
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-271
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:25:49Z · advance · plan → build · native:approved ^t-7768de
  - native_change: fix-271
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:31:53Z · advance · build → test · native:approved ^t-388b4c
  - native_change: fix-271
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:31:58Z · advance · test → accept · native:verifying ^t-f93076
  - native_change: fix-271
  - gate: native:verifying
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:31:58Z · gate-opened · accept → accept · Implementation review of fix-271 ^t-c6ff97
  - native_change: fix-271
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
