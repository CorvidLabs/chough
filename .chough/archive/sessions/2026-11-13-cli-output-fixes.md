---
kind: session
id: 2026-11-13-cli-output-fixes
aliases: [cli-output-fixes]
objective: "0.6.1: chough next with no open session says so and exits 0, and timing says elapsed wall-clock once"
stage: done
stage_status: archived
iteration: 1
native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
cases:
  - TC-cli-next-with-no-open-session-says-so-and-exits-0
  - TC-history-next-after-done-exits-0-and-status-refuses
  - TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line
observations:
  - 2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0
  - 2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses
  - 2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line
  - 2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0-a87838
  - 2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses-5ec968
  - 2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line-22f376
  - 2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0-3758f7
  - 2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses-a6f491
  - 2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line-24eb81
  - 2026-11-13-the-full-test-suite-passed-734-of-734-across-48-targets-cargo
bugs:
  - 2026-11-12-chough-next-with-no-open-session-exits-1-with-error-no-open
  - 2026-11-12-chough-timing-still-says-elapsed-wall-clock-on-the-human-gate
title: "0.6.1: next with no open session exits 0, and timing says elapsed once"
summary:
  - With no open session, the normal state after done, chough next says so, names chough session new <key> and exits 0, while status still refuses (REQ-cli-021, REQ-history-006); chough timing says elapsed wall-clock once, in its header (REQ-reporting-001).
  - "Verified: the cases red then green, both bugs verified, and the full suite 734 of 734. Next: Leif's review and merge."
closed: 2026-11-13
outcome: done
---
# 0.6.1: chough next with no open session says so and exits 0, and timing says elapsed wall-clock once

## Goal

0.6.1: chough next with no open session says so and exits 0, and timing says elapsed wall-clock once

## Where it stands

- PR 6: next with no open session exits 0, and timing says elapsed wall-clock once (REQ-cli-021, REQ-history-006, REQ-reporting-001); red at 1a57d6f3, green at bc275212, both bugs verified

## Decisions and why

- Leif decided "Approve the intent: next with no session exits 0, and timing says wall-clock once": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Small CLI output fixes: chough next with no open session exits 0 with a plain message (F10); timing says elapsed wall-clock once and drops 'read read-only' (F5)'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Small CLI output fixes: chough next with no open session exits 0 with a plain message (F10); timing says elapsed wall-clock once and drops 'read read-only' (F5)'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once as 0xLeif
- Leif decided "Approve the scope of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once with its corrected file list": "Yes, start (Recommended) / Yes, fix and keep (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Small CLI output fixes: chough next with no open session exits 0 with a plain message (F10); timing says elapsed wall-clock once and drops 'read read-only' (F5)'; relayed by claude-code-t370; and at 2026-11-13T02:53Z 'Yes, fix and keep (Recommended)', answering: 'Chough's paperwork listed too few files for some 0.6.1 PRs ... Fix the file lists with no code change ... Keep your existing approvals?' (option text: 'One yes covers #242's extra record (its scope and review) and the corrected lists. From now on, t382 checks the lists before every review.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once as 0xLeif
- Leif decided "Approve the scope of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once with its exact file list": "Yes, start (Recommended) / Yes, fix and keep (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Small CLI output fixes: chough next with no open session exits 0 with a plain message (F10); timing says elapsed wall-clock once and drops 'read read-only' (F5)'; relayed by claude-code-t370; and at 2026-11-13T02:53Z 'Yes, fix and keep (Recommended)', answering: 'Chough's paperwork listed too few files for some 0.6.1 PRs ... Fix the file lists with no code change ... Keep your existing approvals?' (option text: 'One yes covers #242's extra record (its scope and review) and the corrected lists. From now on, t382 checks the lists before every review.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once as 0xLeif
- Leif decided "Implementation review of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once": "Approve (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:40Z, answering: 'Approve 0.6.1 PR #248? chough next with no open session (the normal state after done) says so, names chough session new, and exits cleanly. chough timing says elapsed wall-clock once, in its header. 734 of 734 tests pass, with failing tests first.' (option text: 't382 records your review and ships. Then CI runs, then merge it.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change review 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once as 0xLeif with verdict pass

## What was verified

- 6 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0 for TC-cli-next-with-no-open-session-says-so-and-exits-0: passed (exit 0, tests run 1). — 10e8b60780f0 ^2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0-3758f7
- Ran cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses for TC-history-next-after-done-exits-0-and-status-refuses: passed (exit 0, tests run 1). — a36d80a35890 ^2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses-a6f491
- Ran cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines for TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line: passed (exit 0, tests run 1). — ffabe3eb00f8 ^2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line-24eb81
- The full test suite passed 734 of 734 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 4001748c, PR 6 with main merged in (#243); clippy --all-targets -D warnings and rustfmt --check were clean ^2026-11-13-the-full-test-suite-passed-734-of-734-across-48-targets-cargo

## Bugs

- [chough next with no open session exits 1 with Error: no open session](../../bugs/2026-11-12-chough-next-with-no-open-session-exits-1-with-error-no-open.md): verified
- [chough timing still says elapsed wall-clock on the human-gate line, and its notes say read read-only](../../bugs/2026-11-12-chough-timing-still-says-elapsed-wall-clock-on-the-human-gate.md): verified

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
- 2026-11-13T03:01:29Z · direct-native-write · build → build · 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-080f50
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - revision: 95aebfd72ecb84c4811ed921027520bab991f43a
  - direct_write:
    - change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: claude-code (hc t382)
- 2026-11-13T03:01:29Z · back · build → plan · Leif chose to fix the file list and keep his approvals (02:53Z, relayed by t370) ^t-98868b
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - revision: 95aebfd72ecb84c4811ed921027520bab991f43a
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, checkpoint references: 3 case(s) and 6 observation(s) to re-check with `chough status`
  - by: claude-code (hc t382)
- 2026-11-13T03:01:30Z · gate-opened · plan → plan · Approve the scope of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once with its corrected file list ^t-75b445
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - revision: 95aebfd72ecb84c4811ed921027520bab991f43a
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T03:01:30Z · gate-closed · plan → plan · Approve the scope of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once with its corrected file list ^t-03f1be
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - revision: 95aebfd72ecb84c4811ed921027520bab991f43a
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended) / Yes, fix and keep (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Small CLI output fixes: chough next with no open session exits 0 with a plain message (F10); timing says elapsed wall-clock once and drops 'read read-only' (F5)'; relayed by claude-code-t370; and at 2026-11-13T02:53Z 'Yes, fix and keep (Recommended)', answering: 'Chough's paperwork listed too few files for some 0.6.1 PRs ... Fix the file lists with no code change ... Keep your existing approvals?' (option text: 'One yes covers #242's extra record (its scope and review) and the corrected lists. From now on, t382 checks the lists before every review.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T03:01:32Z · advance · plan → build · Leif kept his approval with the corrected file list (02:53Z, relayed by t370) ^t-af93c2
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - gate: native:approved
  - revision: 95aebfd72ecb84c4811ed921027520bab991f43a
  - by: claude-code (hc t382)
- 2026-11-13T03:01:45Z · advance · build → test · The fixes are green ^t-b08bb3
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - gate: native:approved
  - revision: a0a2b49ac4030dd765050567ccf82c7ae1a67c7e
  - by: claude-code (hc t382)
- 2026-11-13T03:01:45Z · advance · test → accept · The fixes are green ^t-37d950
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - gate: native:approved
  - revision: a0a2b49ac4030dd765050567ccf82c7ae1a67c7e
  - by: claude-code (hc t382)
- 2026-11-13T03:14:32Z · direct-native-write · accept → accept · 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-b3e911
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - revision: 5f68ad78cdfefb6bf2025c2986b24a7a5b578265
  - direct_write:
    - change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: claude-code (hc t382)
- 2026-11-13T03:14:32Z · back · accept → plan · The file list names exactly the changed files (Leif, 02:53Z, relayed by t370) ^t-d30b2d
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - revision: 5f68ad78cdfefb6bf2025c2986b24a7a5b578265
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, test: rerun affected cases; earlier observations may be stale, accept: native verification and the human implementation review, checkpoint references: 3 case(s) and 6 observation(s) to re-check with `chough status`
  - by: claude-code (hc t382)
- 2026-11-13T03:14:33Z · gate-opened · plan → plan · Approve the scope of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once with its exact file list ^t-8dc4a0
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - revision: 5f68ad78cdfefb6bf2025c2986b24a7a5b578265
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T03:14:33Z · gate-closed · plan → plan · Approve the scope of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once with its exact file list ^t-e61693
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - revision: 5f68ad78cdfefb6bf2025c2986b24a7a5b578265
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended) / Yes, fix and keep (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Small CLI output fixes: chough next with no open session exits 0 with a plain message (F10); timing says elapsed wall-clock once and drops 'read read-only' (F5)'; relayed by claude-code-t370; and at 2026-11-13T02:53Z 'Yes, fix and keep (Recommended)', answering: 'Chough's paperwork listed too few files for some 0.6.1 PRs ... Fix the file lists with no code change ... Keep your existing approvals?' (option text: 'One yes covers #242's extra record (its scope and review) and the corrected lists. From now on, t382 checks the lists before every review.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T03:14:35Z · advance · plan → build · Leif kept his approval with the exact file list (02:53Z, relayed by t370) ^t-d4f057
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - gate: native:approved
  - revision: 5f68ad78cdfefb6bf2025c2986b24a7a5b578265
  - by: claude-code (hc t382)
- 2026-11-13T03:14:35Z · advance · build → test · Leif kept his approval with the exact file list (02:53Z, relayed by t370) ^t-fc5685
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - gate: native:approved
  - revision: 5f68ad78cdfefb6bf2025c2986b24a7a5b578265
  - by: claude-code (hc t382)
- 2026-11-13T03:14:35Z · advance · test → accept · Leif kept his approval with the exact file list (02:53Z, relayed by t370) ^t-36b0b1
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - gate: native:approved
  - revision: 5f68ad78cdfefb6bf2025c2986b24a7a5b578265
  - by: claude-code (hc t382)
- 2026-11-13T03:38:41Z · gate-opened · accept → accept · Implementation review of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once ^t-7379a8
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - revision: 4001748c5694f643883219e37d690c4271bfc064
  - holds: accept
  - drives: review
  - by: claude-code (hc t382)
- 2026-11-13T03:42:00Z · gate-closed · accept → accept · Implementation review of 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once ^t-3f6e86
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - revision: d7ca4e5f2a1bee9dfc416c7dc75b6d2f8c941c68
  - relayed_by:
    - decided_by: Leif
    - words: Approve (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:40Z, answering: 'Approve 0.6.1 PR #248? chough next with no open session (the normal state after done) says so, names chough session new, and exits cleanly. chough timing says elapsed wall-clock once, in its header. 734 of 734 tests pass, with failing tests first.' (option text: 't382 records your review and ships. Then CI runs, then merge it.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change review 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T03:42:01Z · advance · accept → done · native:archived ^t-840077
  - native_change: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once
  - gate: native:archived
  - revision: d7ca4e5f2a1bee9dfc416c7dc75b6d2f8c941c68
  - by: claude-code (hc t382)

Closed 2026-11-13: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0
    claim: "Ran cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0 for TC-cli-next-with-no-open-session-says-so-and-exits-0: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0
    revision: 1a57d6f323ee083aac99cc100762d0169f2a7c2a
    environment: local-macos-arm64
    content: 4a0504b923b86eb60de2e722b0fa05dcd4ce59cdea6d267a4415927f86ca2f33
    fingerprint: sha256:8edd6b839e7113d19e3b5c56c20f6a8bf30e0cf17b6856d43dbfbf6b4fd980e1
    case: TC-cli-next-with-no-open-session-says-so-and-exits-0
    case_digest: 3b27295d134205f159c1e25e54aac9aedf67971962308394c42ad1aa59335322
    observed: 2026-11-13T02:56:52Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses
    claim: "Ran cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses for TC-history-next-after-done-exits-0-and-status-refuses: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses
    revision: 1a57d6f323ee083aac99cc100762d0169f2a7c2a
    environment: local-macos-arm64
    content: 4a0504b923b86eb60de2e722b0fa05dcd4ce59cdea6d267a4415927f86ca2f33
    fingerprint: sha256:8f4abf71a0b70d530dc4f62ec40b47d57f496bbd8dc63b67ae1c17df87f971f3
    case: TC-history-next-after-done-exits-0-and-status-refuses
    case_digest: 670778eb5f1bd0d61fd514d67b3ab56c48492a1b0ddc15f7321937a81f540712
    observed: 2026-11-13T02:56:55Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line
    claim: "Ran cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines for TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines
    revision: 1a57d6f323ee083aac99cc100762d0169f2a7c2a
    environment: local-macos-arm64
    content: 4a0504b923b86eb60de2e722b0fa05dcd4ce59cdea6d267a4415927f86ca2f33
    fingerprint: sha256:8ad107876f1fe089476924b718b9b9c9696ea3751ded7ce3b4846694f29d3ac5
    case: TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line
    case_digest: 73ef751d996c495ef5c2c2b09e4780498f345a9ee29cb9e0e0f09a4d3d9c0c5c
    observed: 2026-11-13T02:56:57Z
    duration_seconds: 0
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0-a87838
    claim: "Ran cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0 for TC-cli-next-with-no-open-session-says-so-and-exits-0: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0
    revision: bc275212d81eb8ac9c8498f189809431ab75340c
    environment: local-macos-arm64
    content: 289ed18f7eeb791ead6c5c6ddb8da209cbcde473ed6022b7b1aa7c4b4dbb7082
    fingerprint: sha256:784bc8b77e7c5854c2d094373ad1032ead439032e003eae4861b945e363b96e1
    case: TC-cli-next-with-no-open-session-says-so-and-exits-0
    case_digest: 3b27295d134205f159c1e25e54aac9aedf67971962308394c42ad1aa59335322
    observed: 2026-11-13T03:00:58Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses-5ec968
    claim: "Ran cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses for TC-history-next-after-done-exits-0-and-status-refuses: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses
    revision: bc275212d81eb8ac9c8498f189809431ab75340c
    environment: local-macos-arm64
    content: 289ed18f7eeb791ead6c5c6ddb8da209cbcde473ed6022b7b1aa7c4b4dbb7082
    fingerprint: sha256:5dd4912480a0029ff98ebd3986541ac4f6f790689d49ded4dc017f146d561bfd
    case: TC-history-next-after-done-exits-0-and-status-refuses
    case_digest: 670778eb5f1bd0d61fd514d67b3ab56c48492a1b0ddc15f7321937a81f540712
    observed: 2026-11-13T03:01:01Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line-22f376
    claim: "Ran cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines for TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines
    revision: bc275212d81eb8ac9c8498f189809431ab75340c
    environment: local-macos-arm64
    content: 289ed18f7eeb791ead6c5c6ddb8da209cbcde473ed6022b7b1aa7c4b4dbb7082
    fingerprint: sha256:bbdbe4fbff595dd72e9a1856d94d0ccec190ae8c4c9edeb445584c54b77541ac
    case: TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line
    case_digest: 73ef751d996c495ef5c2c2b09e4780498f345a9ee29cb9e0e0f09a4d3d9c0c5c
    observed: 2026-11-13T03:01:04Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0-3758f7
    claim: "Ran cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0 for TC-cli-next-with-no-open-session-says-so-and-exits-0: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0
    revision: 4001748c5694f643883219e37d690c4271bfc064
    environment: local-macos-arm64
    content: 489d14fd5c396a13d756cc137bef855dfad477971358d696dc797684d4e06219
    fingerprint: sha256:10e8b60780f0059224613d252c68c94b7273fafb7ebc3c3e275a804396f7b764
    case: TC-cli-next-with-no-open-session-says-so-and-exits-0
    case_digest: 3b27295d134205f159c1e25e54aac9aedf67971962308394c42ad1aa59335322
    observed: 2026-11-13T03:30:55Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses-a6f491
    claim: "Ran cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses for TC-history-next-after-done-exits-0-and-status-refuses: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses
    revision: 4001748c5694f643883219e37d690c4271bfc064
    environment: local-macos-arm64
    content: 489d14fd5c396a13d756cc137bef855dfad477971358d696dc797684d4e06219
    fingerprint: sha256:a36d80a35890677ef203302c5e2931e259b74c64e4cf9f0a18fa27fd2a2e847b
    case: TC-history-next-after-done-exits-0-and-status-refuses
    case_digest: 670778eb5f1bd0d61fd514d67b3ab56c48492a1b0ddc15f7321937a81f540712
    observed: 2026-11-13T03:30:59Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line-24eb81
    claim: "Ran cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines for TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines
    revision: 4001748c5694f643883219e37d690c4271bfc064
    environment: local-macos-arm64
    content: 489d14fd5c396a13d756cc137bef855dfad477971358d696dc797684d4e06219
    fingerprint: sha256:ffabe3eb00f8c4f28eec9dbc9d7d61228048d32aad6c52b69220d771fecec013
    case: TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line
    case_digest: 73ef751d996c495ef5c2c2b09e4780498f345a9ee29cb9e0e0f09a4d3d9c0c5c
    observed: 2026-11-13T03:31:03Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-the-full-test-suite-passed-734-of-734-across-48-targets-cargo
    claim: The full test suite passed 734 of 734 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 4001748c, PR 6 with main merged in (#243); clippy --all-targets -D warnings and rustfmt --check were clean
    result: passed
    revision: 4001748c5694f643883219e37d690c4271bfc064
    environment: local-macos-arm64
    content: 489d14fd5c396a13d756cc137bef855dfad477971358d696dc797684d4e06219
    observed: 2026-11-13T03:38:39Z
    actor: claude-code (hc t382)
```
