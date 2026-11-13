---
kind: session
id: 2026-11-13-cli-output-fixes
aliases: [cli-output-fixes]
objective: "0.6.1: chough next with no open session says so and exits 0, and timing says elapsed wall-clock once"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
pr: ""
criteria: []
requirements: []
cases:
  - TC-cli-next-with-no-open-session-says-so-and-exits-0
  - TC-history-next-after-done-exits-0-and-status-refuses
  - TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line
design: []
files: []
observations:
  - 2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0
  - 2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses
  - 2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line
  - 2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0-a87838
  - 2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses-5ec968
  - 2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line-22f376
bugs:
  - 2026-11-12-chough-next-with-no-open-session-exits-1-with-error-no-open
  - 2026-11-12-chough-timing-still-says-elapsed-wall-clock-on-the-human-gate
blockers: []
waiting_on_human: []
run_consents: []
next: Continue build
---
# 0.6.1: chough next with no open session says so and exits 0, and timing says elapsed wall-clock once

## Goal

0.6.1: chough next with no open session says so and exits 0, and timing says elapsed wall-clock once

## Where it stands

- PR 6: next with no open session exits 0, and timing says elapsed wall-clock once (REQ-cli-021, REQ-history-006, REQ-reporting-001); red at 1a57d6f3, green at bc275212, both bugs verified

## Decisions and why

- Leif decided "Approve the intent: next with no session exits 0, and timing says wall-clock once": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Small CLI output fixes: chough next with no open session exits 0 with a plain message (F10); timing says elapsed wall-clock once and drops 'read read-only' (F5)'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Small CLI output fixes: chough next with no open session exits 0 with a plain message (F10); timing says elapsed wall-clock once and drops 'read read-only' (F5)'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once as 0xLeif

## What was verified

- [Ran cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0 for TC-cli-next-with-no-open-session-says-so-and-exits-0: failed (exit 101, tests run 1).](evidence/2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0.md)
- [Ran cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses for TC-history-next-after-done-exits-0-and-status-refuses: failed (exit 101, tests run 1).](evidence/2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses.md)
- [Ran cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines for TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line: failed (exit 101, tests run 1).](evidence/2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line.md)
- [Ran cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0 for TC-cli-next-with-no-open-session-says-so-and-exits-0: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0-a87838.md)
- [Ran cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses for TC-history-next-after-done-exits-0-and-status-refuses: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses-5ec968.md)
- [Ran cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines for TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line-22f376.md)

## Open threads

## Timeline

- 2026-11-13T02:29:18Z · start · intent → intent · 0.6.1: chough next with no open session says so and exits 0, and timing says elapsed wall-clock once ^t-e1a123
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:29:18Z · gate-opened · intent → intent · Approve the intent: next with no session exits 0, and timing says wall-clock once ^t-e95fbe
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: intent
  - drives: none
  - by: claude-code (hc t382)
- 2026-11-13T02:29:18Z · gate-closed · intent → intent · Approve the intent: next with no session exits 0, and timing says wall-clock once ^t-d68f9e
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Small CLI output fixes: chough next with no open session exits 0 with a plain message (F10); timing says elapsed wall-clock once and drops 'read read-only' (F5)'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
  - by: claude-code (hc t382)
- 2026-11-13T02:29:18Z · advance · intent → design · Leif approved 0.6.1's PRs (02:03Z, relayed by t370) ^t-370e9c
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:29:18Z · skip · design → plan · No visuals or interactions change ^t-f609ee
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:29:59Z · gate-opened · plan → plan · Approve the scope of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once ^t-43401a
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T02:29:59Z · gate-closed · plan → plan · Approve the scope of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once ^t-d58e78
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Small CLI output fixes: chough next with no open session exits 0 with a plain message (F10); timing says elapsed wall-clock once and drops 'read read-only' (F5)'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T02:30:02Z · advance · plan → build · Leif approved the scope (02:03Z, relayed by t370) ^t-07c7db
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - gate: native:approved
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
