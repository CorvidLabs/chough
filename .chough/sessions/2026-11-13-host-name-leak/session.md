---
kind: session
id: 2026-11-13-host-name-leak
aliases: [host-name-leak]
objective: "0.6.1: the leak check does not read an ordinary word equal to a short host name as a host-name leak"
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
pr: ""
criteria: []
requirements: []
cases: [TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host]
design: []
files: []
observations:
  - 2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or
  - 2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-3e8be4
  - 2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-a59825
  - 2026-11-13-the-full-test-suite-passed-730-of-730-across-48-targets-cargo
bugs: [2026-11-12-the-leak-check-reads-an-ordinary-word-equal-to-a-short-host]
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
title: "0.6.1: a short host name in an ordinary word is not a leak"
summary:
  - The leak check no longer reads an ordinary word equal to a short all-letter host name as a host-name leak; such a name is a leak only inside a path or a URL or as <user>@<host>, and a longer or hyphenated host name stays a leak anywhere (REQ-records-006).
  - "Verified: the case red then green, the bug verified, and the full suite 730 of 730. Next: Leif's review and merge."
---
# 0.6.1: the leak check does not read an ordinary word equal to a short host name as a host-name leak

## Goal

0.6.1: the leak check does not read an ordinary word equal to a short host name as a host-name leak

## Where it stands

- PR 1: a short all-letter host name is a leak only inside a path, a URL or <user>@<host> (REQ-records-006); red at a6e35a2a, green at 2afedd91, bug verified

## Decisions and why

- Leif decided "Approve the intent: the leak check reads no short host name as a leak in ordinary words": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Leak check: an ordinary word equal to a short host name is not a leak (REQ-records-006 text)'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Leak check: an ordinary word equal to a short host name is not a leak (REQ-records-006 text)'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak as 0xLeif
- Leif decided "Implementation review of 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak": "Approve (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:26Z, answering the coordinator's summary of PR #241, the leak check that stops flagging short machine names in normal sentences (option text: 't382 records your review and ships. Then CI runs, then merge it.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change review 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host for TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host: failed (exit 101).](evidence/2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or.md)
- [Ran cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host for TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host: passed (exit 0).](evidence/2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-3e8be4.md)
- [Ran cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host for TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host: passed (exit 0).](evidence/2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-a59825.md)
- [The full test suite passed 730 of 730 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 2afedd91 with REQ-records-006's new text materialized; clippy --all-targets -D warnings and rustfmt --check were clean](evidence/2026-11-13-the-full-test-suite-passed-730-of-730-across-48-targets-cargo.md)

## Open threads

## Timeline

- 2026-11-13T02:06:24Z · start · intent → intent · 0.6.1: the leak check does not read an ordinary word equal to a short host name as a host-name leak ^t-f30fda
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:06:24Z · gate-opened · intent → intent · Approve the intent: the leak check reads no short host name as a leak in ordinary words ^t-c8e8c7
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: intent
  - drives: none
  - by: claude-code (hc t382)
- 2026-11-13T02:06:36Z · gate-closed · intent → intent · Approve the intent: the leak check reads no short host name as a leak in ordinary words ^t-40fc68
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Leak check: an ordinary word equal to a short host name is not a leak (REQ-records-006 text)'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
  - by: claude-code (hc t382)
- 2026-11-13T02:06:37Z · advance · intent → design · Leif approved 0.6.1's PRs (02:03Z, relayed by t370) ^t-450f5b
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:06:37Z · skip · design → plan · No visuals or interactions change ^t-e450e9
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:07:43Z · gate-opened · plan → plan · Approve the scope of 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak ^t-a7deb0
  - native_change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T02:07:43Z · gate-closed · plan → plan · Approve the scope of 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak ^t-e2c255
  - native_change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Leak check: an ordinary word equal to a short host name is not a leak (REQ-records-006 text)'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T02:07:44Z · advance · plan → build · Leif approved the scope (02:03Z, relayed by t370) ^t-fcd5f7
  - native_change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
  - gate: native:approved
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:15:18Z · advance · build → test · The fix is green ^t-2ac949
  - native_change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
  - gate: native:approved
  - revision: 2afedd91aadf075a51e1933849c57d28b0273a09
  - by: claude-code (hc t382)
- 2026-11-13T02:15:18Z · advance · test → accept · The fix is green ^t-67c270
  - native_change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
  - gate: native:approved
  - revision: 2afedd91aadf075a51e1933849c57d28b0273a09
  - by: claude-code (hc t382)
- 2026-11-13T02:23:21Z · gate-opened · accept → accept · Implementation review of 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak ^t-d30478
  - native_change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
  - revision: 2afedd91aadf075a51e1933849c57d28b0273a09
  - holds: accept
  - drives: review
  - by: claude-code (hc t382)
- 2026-11-13T02:27:11Z · gate-closed · accept → accept · Implementation review of 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak ^t-de7ef0
  - native_change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
  - revision: 25acc4810745bbb8976c79db4ebd9b78f1dcd1dd
  - relayed_by:
    - decided_by: Leif
    - words: Approve (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:26Z, answering the coordinator's summary of PR #241, the leak check that stops flagging short machine names in normal sentences (option text: 't382 records your review and ships. Then CI runs, then merge it.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change review 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T02:27:13Z · advance · accept → done · native:archived ^t-b87a31
  - native_change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
  - gate: native:archived
  - revision: 25acc4810745bbb8976c79db4ebd9b78f1dcd1dd
  - by: claude-code (hc t382)
