---
kind: session
id: 2026-11-13-change-ship-output
aliases: [change-ship-output]
objective: "0.6.1: chough change ship prints a short text summary after SpecSync's output, with its full report under --json"
stage: done
stage_status: archived
iteration: 1
native_change: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
cases: [TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json]
observations:
  - 2026-11-13-tc-cli-change-ship-prints-a-short-summary-and-its-reports
  - 2026-11-13-tc-cli-change-ship-prints-a-short-summary-and-its-reports-a90344
  - 2026-11-13-tc-cli-change-ship-prints-a-short-summary-and-its-reports-163898
  - 2026-11-13-the-full-test-suite-passed-730-of-730-across-48-targets-cargo
  - 2026-11-13-the-full-test-suite-passed-731-of-731-across-48-targets-cargo
bugs: [2026-11-12-chough-change-ship-prints-specsync-s-text-a-long-json]
title: "0.6.1: chough change ship prints a short summary"
summary:
  - After SpecSync's output, chough change ship prints a short summary of the review, the ship, the gate close and the move into done, at most 12 lines, with every step's full report under --json (REQ-cli-059).
  - "Verified: the case red then green, the bug verified, and the full suite 730 of 730. Next: Leif's review and merge."
closed: 2026-11-13
outcome: done
---
# 0.6.1: chough change ship prints a short text summary after SpecSync's output, with its full report under --json

## Goal

0.6.1: chough change ship prints a short text summary after SpecSync's output, with its full report under --json

## Where it stands

- PR 3: change ship prints a short summary after SpecSync's output, its reports under --json (REQ-cli-059); red at c29cf1e6, green at 22568b99, bug verified

## Decisions and why

- Leif decided "Approve the intent: change ship prints a short summary, its full report behind --json": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'chough change ship prints a short text summary, with its full JSON under --json, as stage moves already do'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'chough change ship prints a short text summary, with its full JSON under --json, as stage moves already do'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json as 0xLeif
- Leif decided "Implementation review of 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json": "Approve (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:44Z, answering: 'Approve 0.6.1 PR #243? chough change ship prints at most 12 lines (review, ship, gate close, move into done), with the full detail under --json. Only src/change.rs changed, with a test that failed first.' (option text: 't382 records your review and ships. Then CI runs, then merge it.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change review 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json as 0xLeif with verdict pass

## What was verified

- 2 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json for TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json: passed (exit 0, tests run 1). — b4e03cee984e ^2026-11-13-tc-cli-change-ship-prints-a-short-summary-and-its-reports-163898
- The full test suite passed 730 of 730 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 22568b99 with REQ-cli-059's new text materialized; clippy --all-targets -D warnings and rustfmt --check were clean ^2026-11-13-the-full-test-suite-passed-730-of-730-across-48-targets-cargo
- The full test suite passed 731 of 731 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 595d0930, #243's final head with main merged in; clippy --all-targets -D warnings and rustfmt --check were clean ^2026-11-13-the-full-test-suite-passed-731-of-731-across-48-targets-cargo

## Bugs

- [chough change ship prints SpecSync's text, a long JSON checkpoint and the move summary together](../../bugs/2026-11-12-chough-change-ship-prints-specsync-s-text-a-long-json.md): verified

## Open threads

## Timeline

- 2026-11-13T02:15:49Z · start · intent → intent · 0.6.1: chough change ship prints a short text summary after SpecSync's output, with its full report under --json ^t-ed43b4
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:15:49Z · gate-opened · intent → intent · Approve the intent: change ship prints a short summary, its full report behind --json ^t-03986a
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: intent
  - drives: none
  - by: claude-code (hc t382)
- 2026-11-13T02:15:49Z · gate-closed · intent → intent · Approve the intent: change ship prints a short summary, its full report behind --json ^t-8783c5
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'chough change ship prints a short text summary, with its full JSON under --json, as stage moves already do'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
  - by: claude-code (hc t382)
- 2026-11-13T02:15:49Z · advance · intent → design · Leif approved 0.6.1's PRs (02:03Z, relayed by t370) ^t-2ef196
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:15:49Z · skip · design → plan · No visuals or interactions change ^t-7a1743
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:16:18Z · gate-opened · plan → plan · Approve the scope of 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json ^t-8aefa8
  - native_change: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T02:16:18Z · gate-closed · plan → plan · Approve the scope of 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json ^t-d18e63
  - native_change: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'chough change ship prints a short text summary, with its full JSON under --json, as stage moves already do'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T02:16:20Z · advance · plan → build · Leif approved the scope (02:03Z, relayed by t370) ^t-c3de75
  - native_change: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
  - gate: native:approved
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:33:36Z · advance · build → test · The fix is green ^t-3300e3
  - native_change: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
  - gate: native:approved
  - revision: 22568b99de030da4067b1accfac83e2c91d9e993
  - by: claude-code (hc t382)
- 2026-11-13T02:33:36Z · advance · test → accept · The fix is green ^t-764c02
  - native_change: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
  - gate: native:approved
  - revision: 22568b99de030da4067b1accfac83e2c91d9e993
  - by: claude-code (hc t382)
- 2026-11-13T02:42:04Z · gate-opened · accept → accept · Implementation review of 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json ^t-490eb2
  - native_change: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
  - revision: d0fc7e970adb7ed2f55cc8c23c2bbb5d6112fcd9
  - holds: accept
  - drives: review
  - by: claude-code (hc t382)
- 2026-11-13T02:58:40Z · gate-closed · accept → accept · Implementation review of 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json ^t-be498f
  - native_change: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
  - revision: d67d3110cb3acc8a9de698e287d9391c484fa914
  - relayed_by:
    - decided_by: Leif
    - words: Approve (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:44Z, answering: 'Approve 0.6.1 PR #243? chough change ship prints at most 12 lines (review, ship, gate close, move into done), with the full detail under --json. Only src/change.rs changed, with a test that failed first.' (option text: 't382 records your review and ships. Then CI runs, then merge it.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change review 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T02:58:41Z · advance · accept → done · native:archived ^t-c8fd22
  - native_change: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
  - gate: native:archived
  - revision: d67d3110cb3acc8a9de698e287d9391c484fa914
  - by: claude-code (hc t382)

Closed 2026-11-13: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-13-tc-cli-change-ship-prints-a-short-summary-and-its-reports
    claim: "Ran cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json for TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json
    revision: c29cf1e653a31e63748d144c4bf1e22942d57a61
    environment: local-macos-arm64
    content: 0875b1cc72bf9eb8374f1ccf008ceb008770a5be3c01bf0dcae51c961b4c1f1d
    fingerprint: sha256:44c704b7ed2479967990e480c81b83d5c5fc45d4daf477704004e8685db7873e
    case: TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json
    case_digest: 76aff66faf917a27b83922b3af28cfb3cca4f54d8298905dc1fd0bf54d26f3e6
    observed: 2026-11-13T02:32:47Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-change-ship-prints-a-short-summary-and-its-reports-a90344
    claim: "Ran cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json for TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json
    revision: 22568b99de030da4067b1accfac83e2c91d9e993
    environment: local-macos-arm64
    content: 9a72d59c509bca86133978ea0ed478c12eea1e036835535143167fc9eaabbbc3
    fingerprint: sha256:b948e881caf978de10fe5b323d8d5af5204d7a0a13915c0571296eba4d40a8bb
    case: TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json
    case_digest: 76aff66faf917a27b83922b3af28cfb3cca4f54d8298905dc1fd0bf54d26f3e6
    observed: 2026-11-13T02:33:25Z
    duration_seconds: 3
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-change-ship-prints-a-short-summary-and-its-reports-163898
    claim: "Ran cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json for TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json
    revision: d0fc7e970adb7ed2f55cc8c23c2bbb5d6112fcd9
    environment: local-macos-arm64
    content: b98eab13fdc77afc8782e0d29cb271be64181bed096c9014b27bebff794e3366
    fingerprint: sha256:b4e03cee984ed03e5614e3ca8ab943d90a0b903991cf28ae3da2a9f7eb637a39
    case: TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json
    case_digest: 76aff66faf917a27b83922b3af28cfb3cca4f54d8298905dc1fd0bf54d26f3e6
    observed: 2026-11-13T02:41:36Z
    duration_seconds: 2
    actor: claude-code (hc t382)
  - id: 2026-11-13-the-full-test-suite-passed-730-of-730-across-48-targets-cargo
    claim: The full test suite passed 730 of 730 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 22568b99 with REQ-cli-059's new text materialized; clippy --all-targets -D warnings and rustfmt --check were clean
    result: passed
    revision: d0fc7e970adb7ed2f55cc8c23c2bbb5d6112fcd9
    environment: local-macos-arm64
    content: b98eab13fdc77afc8782e0d29cb271be64181bed096c9014b27bebff794e3366
    observed: 2026-11-13T02:42:02Z
    actor: claude-code (hc t382)
  - id: 2026-11-13-the-full-test-suite-passed-731-of-731-across-48-targets-cargo
    claim: "The full test suite passed 731 of 731 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 595d0930, #243's final head with main merged in; clippy --all-targets -D warnings and rustfmt --check were clean"
    result: passed
    revision: 595d09309e6644f46c0608e8d91bab5ff9db6e60
    environment: local-macos-arm64
    content: b98eab13fdc77afc8782e0d29cb271be64181bed096c9014b27bebff794e3366
    observed: 2026-11-13T02:57:54Z
    actor: claude-code (hc t382)
```
