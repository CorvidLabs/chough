---
kind: session
id: 2026-11-13-host-name-leak
aliases: [host-name-leak]
objective: "0.6.1: the leak check does not read an ordinary word equal to a short host name as a host-name leak"
stage: done
stage_status: archived
iteration: 1
native_change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
cases: [TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host]
observations:
  - 2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or
  - 2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-3e8be4
  - 2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-a59825
  - 2026-11-13-the-full-test-suite-passed-730-of-730-across-48-targets-cargo
bugs: [2026-11-12-the-leak-check-reads-an-ordinary-word-equal-to-a-short-host]
title: "0.6.1: a short host name in an ordinary word is not a leak"
summary:
  - The leak check no longer reads an ordinary word equal to a short all-letter host name as a host-name leak; such a name is a leak only inside a path or a URL or as <user>@<host>, and a longer or hyphenated host name stays a leak anywhere (REQ-records-006).
  - "Verified: the case red then green, the bug verified, and the full suite 730 of 730. Next: Leif's review and merge."
closed: 2026-11-13
outcome: done
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

- 2 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host for TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host: passed (exit 0). — 4e56ccfdc2ba ^2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-a59825
- The full test suite passed 730 of 730 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 2afedd91 with REQ-records-006's new text materialized; clippy --all-targets -D warnings and rustfmt --check were clean ^2026-11-13-the-full-test-suite-passed-730-of-730-across-48-targets-cargo

## Bugs

- [The leak check reads an ordinary word equal to a short host name as a host-name leak](../../bugs/2026-11-12-the-leak-check-reads-an-ordinary-word-equal-to-a-short-host.md): verified

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

Closed 2026-11-13: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or
    claim: "Ran cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host for TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host: failed (exit 101)."
    result: failed
    command: cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host
    revision: a6e35a2a2fc53c663b67ad9e1365ed93a75aca3a
    environment: local-macos-arm64
    content: 2aa7ebcd7186df65339001a98dedfc0f1449098bd6863b966f69547c0c2137af
    fingerprint: sha256:683bb23c2fea47be250634c49e352e23b8f41ece666bf6fbe4d5f0c68f206b3d
    case: TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host
    case_digest: c88004f0d6ff74b260d77b5808d0ca8cfdfc535ac0d65f6f96d375fd6a50ecaa
    observed: 2026-11-13T02:14:49Z
    duration_seconds: 0
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-3e8be4
    claim: "Ran cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host for TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host
    revision: 2afedd91aadf075a51e1933849c57d28b0273a09
    environment: local-macos-arm64
    content: 1924becf8e77d3ef9100282e20b749296511dc9aa70163bf32933bd51fb04c51
    fingerprint: sha256:68eccee8e3b570361995e8d1921ead73b478545cae652bad9f0b2a5702be6538
    case: TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host
    case_digest: c88004f0d6ff74b260d77b5808d0ca8cfdfc535ac0d65f6f96d375fd6a50ecaa
    observed: 2026-11-13T02:15:02Z
    duration_seconds: 0
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-a59825
    claim: "Ran cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host for TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host
    revision: 2afedd91aadf075a51e1933849c57d28b0273a09
    environment: local-macos-arm64
    content: e6ec921d16e0556b7c5eee7012581b903d4bce5df7161fa14a82466e480e9c6a
    fingerprint: sha256:4e56ccfdc2badbd4c3599a576438478b010dfafcdc96e382ad67f979971de04d
    case: TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host
    case_digest: c88004f0d6ff74b260d77b5808d0ca8cfdfc535ac0d65f6f96d375fd6a50ecaa
    observed: 2026-11-13T02:23:14Z
    duration_seconds: 0
    actor: claude-code (hc t382)
  - id: 2026-11-13-the-full-test-suite-passed-730-of-730-across-48-targets-cargo
    claim: The full test suite passed 730 of 730 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 2afedd91 with REQ-records-006's new text materialized; clippy --all-targets -D warnings and rustfmt --check were clean
    result: passed
    revision: 2afedd91aadf075a51e1933849c57d28b0273a09
    environment: local-macos-arm64
    content: e6ec921d16e0556b7c5eee7012581b903d4bce5df7161fa14a82466e480e9c6a
    observed: 2026-11-13T02:23:14Z
    actor: claude-code (hc t382)
```
