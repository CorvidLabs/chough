---
kind: session
id: 2026-11-13-flake-window
aliases: [flake-window]
objective: "#217: fledge run flake-window computes Chough's two-week clean window from CI history, flake issues and bug records (REQ-cli-062)"
intent_scope: LIFE
modules: [cli]
stage: done
stage_status: archived
iteration: 1
native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
cases:
  - TC-cli-flake-window-clean
  - TC-cli-flake-window-restarts
  - TC-cli-flake-window-blocked
  - TC-cli-flake-window-fails-closed
  - TC-cli-flake-window-wiring
files:
  - .github/scripts/test-flake-window.py
  - .github/scripts/flake-window.py
  - fledge.toml
  - .github/workflows/ci.yml
  - docs/ci.md
  - docs/releasing.md
  - docs/test-plan-1.0.md
  - CHANGELOG.md
observations:
  - 2026-11-13-tc-cli-flake-window-clean
  - 2026-11-13-tc-cli-flake-window-restarts
  - 2026-11-13-tc-cli-flake-window-blocked
  - 2026-11-13-tc-cli-flake-window-fails-closed
  - 2026-11-13-tc-cli-flake-window-wiring
  - 2026-11-13-tc-cli-flake-window-clean-356549
  - 2026-11-13-tc-cli-flake-window-restarts-5cbeb4
  - 2026-11-13-tc-cli-flake-window-blocked-d43986
  - 2026-11-13-tc-cli-flake-window-fails-closed-e42a32
  - 2026-11-13-tc-cli-flake-window-wiring-b968fa
  - 2026-11-13-the-check-read-live-ci-history-for-corvidlabs-chough-in-27-6-s
  - 2026-11-13-the-full-test-suite-passed-735-of-735-across-48-targets-cargo
  - 2026-11-13-tc-cli-flake-window-restarts-ec7983
  - 2026-11-13-tc-cli-flake-window-fails-closed-cbf9e4
  - 2026-11-13-tc-cli-flake-window-clean-5effb3
  - 2026-11-13-tc-cli-flake-window-restarts-a60440
  - 2026-11-13-tc-cli-flake-window-blocked-7a226e
  - 2026-11-13-tc-cli-flake-window-fails-closed-fe148d
  - 2026-11-13-tc-cli-flake-window-wiring-816737
  - 2026-11-13-the-full-test-suite-passed-735-of-735-across-48-targets-at
  - 2026-11-13-live-run-after-the-fix-with-the-flake-labels-on-github-but
bugs: [2026-11-13-flake-window-counts-a-day-covered-when-its-product-jobs-passed]
closed: 2026-11-14
outcome: done
---
# #217: fledge run flake-window computes Chough's two-week clean window from CI history, flake issues and bug records (REQ-cli-062)

## Goal

#217: fledge run flake-window computes Chough's two-week clean window from CI history, flake issues and bug records (REQ-cli-062)

## Where it stands

- REQ-cli-062: flake-window.py, the fledge tasks, the CI fixtures step and the docs; 24 of 24 fixtures pass locally

## Decisions and why

- The check is a repo script plus a fledge task, recorded as evidence at release; Chough itself gains no command and no GitHub dependency (Leif, Q1, 2026-11-13T03:30Z, relayed by t370; rejected: a chough command, a daily GitHub job posting to an issue)
- Any red CI run on main, any rerun on any branch, or a newly filed flake restarts the window (Leif, Q2, 2026-11-13T03:30Z, relayed by t370; rejected: only failures traced to a filed flake)
- Requirement ID REQ-cli-062 (t382, records owner, 2026-11-13); a flake bug record names its issue as 'GitHub #<n>' in a bug update note, since bug update has no criterion flag
- Leif decided "Approve the intent: fledge run flake-window computes the two-week clean window from CI history (#217, bar 2)": "Repo script + fledge task (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q1 'Where should the two-week check live?' for #217 (same answer set: Q2 'What restarts the window?': 'Any red, rerun or new flake (Recommended)'); relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated
- Leif decided "Approve the scope of 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read": "Any red, rerun or new flake (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q2 'What restarts the window?' for #217 (same answer set: Q1 'Where should the two-week check live?': 'Repo script + fledge task (Recommended)'); this change's scope is that design: a read-only repo script and fledge task, REQ-cli-062; relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated; ran specsync change approve 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read as 0xLeif
- 0xLeif decided "Implementation review of 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read": "0xLeif ran: specsync change review 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read --reviewer 0xLeif --verdict pass" (0xLeif's own terminal in the Claude Code desktop app, 2026-11-14; the native review is already recorded at 3be6d7d); recorded by claude-code; a relayed claim, not authenticated; no native review ran (--no-native)

## What was verified

- 12 earlier runs of the cases below are kept in this story's receipts, not listed here.
- The check read live CI history for CorvidLabs/chough in 27.6 s: 808 runs over 45 days, 135 main runs, 134 job lists, 0 flake issues, 205 bug records (3 name DRIVE-19). Verdict running: latest restart the rerun of run 43357893936 on leif/0.6.1-observation-cap at 2026-11-13T03:33:05Z, clean from 2026-11-27T03:33:05Z, 15 restarts in the window. Recomputing from the saved input gave a byte-identical receipt (input sha256 40fe8591bf6c78c0). This shows the check runs on real data, not that the window is clean ^2026-11-13-the-check-read-live-ci-history-for-corvidlabs-chough-in-27-6-s
- The full test suite passed 735 of 735 across 48 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at 383e1ea3, #217's head with main (#248) merged and REQ-cli-062 materialized; the flake-window fixtures passed 24 of 24 and rustfmt --check was clean. No Rust file changed, so clippy was not rerun ^2026-11-13-the-full-test-suite-passed-735-of-735-across-48-targets-cargo
- Ran python3 .github/scripts/test-flake-window.py CleanWindowTests for TC-cli-flake-window-clean: passed (exit 0). — f920ee0e8b42 ^2026-11-13-tc-cli-flake-window-clean-5effb3
- Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: passed (exit 0). — 86337cfd8ed1 ^2026-11-13-tc-cli-flake-window-restarts-a60440
- Ran python3 .github/scripts/test-flake-window.py BlockedTests for TC-cli-flake-window-blocked: passed (exit 0). — 216becf55cbd ^2026-11-13-tc-cli-flake-window-blocked-7a226e
- Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: passed (exit 0). — 0d47e6f94db7 ^2026-11-13-tc-cli-flake-window-fails-closed-fe148d
- Ran python3 .github/scripts/test-flake-window.py WiringTests for TC-cli-flake-window-wiring: passed (exit 0). — 2651f01cb665 ^2026-11-13-tc-cli-flake-window-wiring-816737
- The full test suite passed 735 of 735 across 48 targets at 05786a14, #252's head after t384's must-fix, under the heavy-tests lock; the flake-window fixtures passed 27 of 27 and rustfmt --check was clean. No Rust file changed, so clippy was not rerun ^2026-11-13-the-full-test-suite-passed-735-of-735-across-48-targets-at
- Live run after the fix, with the flake labels on GitHub but this branch's records not seeded: verdict blocked by #132, #169 and #187 having no bug record naming them (the seed PR #255 adds those) and #247 open with its record on its fix branch; 814 runs, 136 main runs, 4 flake issues, 206 bug records. This shows the check on live data, not that the window is clean ^2026-11-13-live-run-after-the-fix-with-the-flake-labels-on-github-but

## Bugs

- [flake window counts a day covered when its Product jobs passed only in different runs](../../bugs/2026-11-13-flake-window-counts-a-day-covered-when-its-product-jobs-passed.md): verified

## Open threads

## Timeline

- 2026-11-13T03:35:00Z · start · intent → intent · #217: fledge run flake-window computes Chough's two-week clean window from CI history, flake issues and bug records (REQ-cli-062) ^t-fe3698
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (hc t38r)
- 2026-11-13T03:35:23Z · gate-opened · intent → intent · Approve the intent: fledge run flake-window computes the two-week clean window from CI history (#217, bar 2) ^t-7625ac
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - holds: intent
  - drives: none
  - by: claude-code (hc t38r)
- 2026-11-13T03:35:42Z · gate-closed · intent → intent · Approve the intent: fledge run flake-window computes the two-week clean window from CI history (#217, bar 2) ^t-d5a7b1
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - relayed_by:
    - decided_by: Leif
    - words: Repo script + fledge task (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q1 'Where should the two-week check live?' for #217 (same answer set: Q2 'What restarts the window?': 'Any red, rerun or new flake (Recommended)'); relayed by claude-code-t370
    - recorded_by: claude-code-t38r
  - by: claude-code (hc t38r)
- 2026-11-13T03:35:42Z · advance · intent → design ·  ^t-77a4e6
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (hc t38r)
- 2026-11-13T03:35:42Z · skip · design → plan · No visuals or interactions change: a read-only repo script and docs ^t-086976
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (hc t38r)
- 2026-11-13T03:38:31Z · gate-opened · plan → plan · Approve the scope of 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read ^t-e9482b
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - holds: plan
  - drives: approve
  - by: claude-code (hc t38r)
- 2026-11-13T03:38:31Z · gate-closed · plan → plan · Approve the scope of 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read ^t-5bb0de
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - relayed_by:
    - decided_by: Leif
    - words: Any red, rerun or new flake (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q2 'What restarts the window?' for #217 (same answer set: Q1 'Where should the two-week check live?': 'Repo script + fledge task (Recommended)'); this change's scope is that design: a read-only repo script and fledge task, REQ-cli-062; relayed by claude-code-t370
    - recorded_by: claude-code-t38r
    - native: specsync change approve 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
    - actor: 0xLeif
  - by: claude-code (hc t38r)
- 2026-11-13T03:38:38Z · advance · plan → build · native:approved ^t-800fe3
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - gate: native:approved
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (hc t38r)
- 2026-11-13T04:12:48Z · advance · build → test · native:approved ^t-f3f272
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - gate: native:approved
  - revision: 0ecf6166900c69420695dd46cfb20fff012e1d1d
  - by: claude-code (hc t38r)
- 2026-11-13T04:12:49Z · advance · test → accept · native:approved ^t-c05b1e
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - gate: native:approved
  - revision: 0ecf6166900c69420695dd46cfb20fff012e1d1d
  - by: claude-code (hc t38r)
- 2026-11-13T04:55:11Z · gate-opened · accept → accept · Implementation review of 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read ^t-9b3ce6
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - revision: 0327bbdde1ec2e8be391c7f81bdf4f3d010c07ac
  - holds: accept
  - drives: review
  - by: claude-code (hc t38r)
- 2026-11-13T05:00:38Z · gate-closed · accept → accept · Implementation review of 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read ^t-957a7a
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - revision: 3287c91f35c7d024b21e7ba423ab15e68d85b0c8
  - by: claude-code (hc t38r)
- 2026-11-13T05:00:40Z · back · accept → build · t384's independent review of #252 at 3287c91f found a mismatch with REQ-cli-062: a day counted as covered when its two Product jobs passed only in different main runs ^t-db2398
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - gate: native:verifying
  - revision: 3287c91f35c7d024b21e7ba423ab15e68d85b0c8
  - refresh: build: implementation against the current definition, test: rerun affected cases; earlier observations may be stale, accept: native verification and the human implementation review, checkpoint references: 5 case(s) and 12 observation(s) to re-check with `chough status`
  - by: claude-code (hc t38r)
- 2026-11-13T05:02:52Z · advance · build → test · native:verifying ^t-2f4562
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - gate: native:verifying
  - revision: 05786a14526a1c168e7ff6678d81ac884abd579f
  - by: claude-code (hc t38r)
- 2026-11-13T05:02:54Z · advance · test → accept · native:verifying ^t-eed87d
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - gate: native:verifying
  - revision: 05786a14526a1c168e7ff6678d81ac884abd579f
  - by: claude-code (hc t38r)
- 2026-11-13T05:52:25Z · gate-opened · accept → accept · Implementation review of 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read ^t-b72c1b
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - revision: 69fccd81a9b17ab3f386c60a71529d3fd656ac76
  - holds: accept
  - drives: review
  - by: claude-code (hc t38r)
- 2026-11-14T17:34:51Z · gate-closed · accept → accept · Implementation review of 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read ^t-6724d0
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - revision: 9740ec2e0bfb6ab467010cfc538faa68be3fdf97
  - relayed_by:
    - decided_by: 0xLeif
    - words: 0xLeif ran: specsync change review 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read --reviewer 0xLeif --verdict pass
    - where: 0xLeif's own terminal in the Claude Code desktop app, 2026-11-14; the native review is already recorded at 3be6d7d
    - recorded_by: claude-code
  - by: claude-code
- 2026-11-14T17:34:52Z · advance · accept → done · native:archived ^t-7d8629
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - gate: native:archived
  - revision: 9740ec2e0bfb6ab467010cfc538faa68be3fdf97
  - by: claude-code

Closed 2026-11-14: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-13-tc-cli-flake-window-clean
    claim: "Ran python3 .github/scripts/test-flake-window.py CleanWindowTests for TC-cli-flake-window-clean: failed (exit 1)."
    result: failed
    command: python3 .github/scripts/test-flake-window.py CleanWindowTests
    revision: 2f6a807d912cc30cf740c99b1b92cfc5896f5756
    environment: local-macos-arm64
    content: 65c737cfb72492c9aff2cfe5ae2aff47f17987e8b55e4eb7297da321befdb1e6
    fingerprint: sha256:c49badcc265d8c506f95a7fdc3fc68ad3e3345ddf55e250ee56df322af6d6f05
    case: TC-cli-flake-window-clean
    case_digest: 5fb48fb0e12045738962b757c647c98d98cc3e5853ffaefa7966dce77e4024b9
    observed: 2026-11-13T03:41:13Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-restarts
    claim: "Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: failed (exit 1)."
    result: failed
    command: python3 .github/scripts/test-flake-window.py RestartTests
    revision: 2f6a807d912cc30cf740c99b1b92cfc5896f5756
    environment: local-macos-arm64
    content: 65c737cfb72492c9aff2cfe5ae2aff47f17987e8b55e4eb7297da321befdb1e6
    fingerprint: sha256:7f93aa456d0d55a79b30d7b8bf790578df85ab53f689dabce906b85712fc82c0
    case: TC-cli-flake-window-restarts
    case_digest: 2ba347ea0fd6652b41287956e98b3f8e4272cb790fd007afb15f7e160805027a
    observed: 2026-11-13T03:41:16Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-blocked
    claim: "Ran python3 .github/scripts/test-flake-window.py BlockedTests for TC-cli-flake-window-blocked: failed (exit 1)."
    result: failed
    command: python3 .github/scripts/test-flake-window.py BlockedTests
    revision: 2f6a807d912cc30cf740c99b1b92cfc5896f5756
    environment: local-macos-arm64
    content: 65c737cfb72492c9aff2cfe5ae2aff47f17987e8b55e4eb7297da321befdb1e6
    fingerprint: sha256:9627d49e16e01bc200ff1fa0fd4e4e71311b4d374b2870e11af08c44d30bd0b2
    case: TC-cli-flake-window-blocked
    case_digest: a480bb1749670a6d79b43f09a94931f954195cc93c5a4928953b9518accdb81b
    observed: 2026-11-13T03:41:18Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-fails-closed
    claim: "Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: failed (exit 1)."
    result: failed
    command: python3 .github/scripts/test-flake-window.py FailClosedTests
    revision: 2f6a807d912cc30cf740c99b1b92cfc5896f5756
    environment: local-macos-arm64
    content: 65c737cfb72492c9aff2cfe5ae2aff47f17987e8b55e4eb7297da321befdb1e6
    fingerprint: sha256:65e35ab3aaa8f293600301e0d87e12c2bb00748b2425863950b43194b03d9bed
    case: TC-cli-flake-window-fails-closed
    case_digest: b41c1406363789e9833fe3673f7285e4aaca6bc9cad9653be174651d49deea4d
    observed: 2026-11-13T03:41:21Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-wiring
    claim: "Ran python3 .github/scripts/test-flake-window.py WiringTests for TC-cli-flake-window-wiring: failed (exit 1)."
    result: failed
    command: python3 .github/scripts/test-flake-window.py WiringTests
    revision: 2f6a807d912cc30cf740c99b1b92cfc5896f5756
    environment: local-macos-arm64
    content: 65c737cfb72492c9aff2cfe5ae2aff47f17987e8b55e4eb7297da321befdb1e6
    fingerprint: sha256:76e8dce46c198774485ceee13712e159630771496e7c235045662c7abdd8d6fb
    case: TC-cli-flake-window-wiring
    case_digest: 03d0f18b6f7c3f3416763c407dffaf5a954527710c901efc8c5952455012c242
    observed: 2026-11-13T03:41:23Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-clean-356549
    claim: "Ran python3 .github/scripts/test-flake-window.py CleanWindowTests for TC-cli-flake-window-clean: passed (exit 0)."
    result: passed
    command: python3 .github/scripts/test-flake-window.py CleanWindowTests
    revision: 1d711eddf2ac335963f51cd802604e839b212a7e
    environment: local-macos-arm64
    content: 552e0adb8469b6e05a2648cdca2cc07d6e9cd0eed92c842f1170d1e653297e27
    fingerprint: sha256:1585fd3811c11e4cba8b310791c888030101c7170b1980e31983330a39dc9e52
    case: TC-cli-flake-window-clean
    case_digest: 5fb48fb0e12045738962b757c647c98d98cc3e5853ffaefa7966dce77e4024b9
    observed: 2026-11-13T03:50:16Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-restarts-5cbeb4
    claim: "Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: passed (exit 0)."
    result: passed
    command: python3 .github/scripts/test-flake-window.py RestartTests
    revision: 1d711eddf2ac335963f51cd802604e839b212a7e
    environment: local-macos-arm64
    content: 552e0adb8469b6e05a2648cdca2cc07d6e9cd0eed92c842f1170d1e653297e27
    fingerprint: sha256:afbd3a6b8c662d0b567f788d3ac47decb966b63939d50ee6b3f5c8e3a37f5288
    case: TC-cli-flake-window-restarts
    case_digest: 2ba347ea0fd6652b41287956e98b3f8e4272cb790fd007afb15f7e160805027a
    observed: 2026-11-13T03:50:20Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-blocked-d43986
    claim: "Ran python3 .github/scripts/test-flake-window.py BlockedTests for TC-cli-flake-window-blocked: passed (exit 0)."
    result: passed
    command: python3 .github/scripts/test-flake-window.py BlockedTests
    revision: 1d711eddf2ac335963f51cd802604e839b212a7e
    environment: local-macos-arm64
    content: 552e0adb8469b6e05a2648cdca2cc07d6e9cd0eed92c842f1170d1e653297e27
    fingerprint: sha256:0dba8d54a5c8a7f28f51a0ed4df710b7fecbbda647bc030fd5bc364915e12ceb
    case: TC-cli-flake-window-blocked
    case_digest: a480bb1749670a6d79b43f09a94931f954195cc93c5a4928953b9518accdb81b
    observed: 2026-11-13T03:50:24Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-fails-closed-e42a32
    claim: "Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: passed (exit 0)."
    result: passed
    command: python3 .github/scripts/test-flake-window.py FailClosedTests
    revision: 1d711eddf2ac335963f51cd802604e839b212a7e
    environment: local-macos-arm64
    content: 552e0adb8469b6e05a2648cdca2cc07d6e9cd0eed92c842f1170d1e653297e27
    fingerprint: sha256:3204661e793c7d18300d40c2d56990aa38e1b5fc8faf05e650ddf9c484c05f8b
    case: TC-cli-flake-window-fails-closed
    case_digest: b41c1406363789e9833fe3673f7285e4aaca6bc9cad9653be174651d49deea4d
    observed: 2026-11-13T03:50:30Z
    duration_seconds: 1
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-wiring-b968fa
    claim: "Ran python3 .github/scripts/test-flake-window.py WiringTests for TC-cli-flake-window-wiring: passed (exit 0)."
    result: passed
    command: python3 .github/scripts/test-flake-window.py WiringTests
    revision: 1d711eddf2ac335963f51cd802604e839b212a7e
    environment: local-macos-arm64
    content: 552e0adb8469b6e05a2648cdca2cc07d6e9cd0eed92c842f1170d1e653297e27
    fingerprint: sha256:fe8ad70cd7d04840fb847fb044ab3213794a24cb07a69ef8461ca6a8f75a7322
    case: TC-cli-flake-window-wiring
    case_digest: 03d0f18b6f7c3f3416763c407dffaf5a954527710c901efc8c5952455012c242
    observed: 2026-11-13T03:50:34Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-the-check-read-live-ci-history-for-corvidlabs-chough-in-27-6-s
    claim: "The check read live CI history for CorvidLabs/chough in 27.6 s: 808 runs over 45 days, 135 main runs, 134 job lists, 0 flake issues, 205 bug records (3 name DRIVE-19). Verdict running: latest restart the rerun of run 43357893936 on leif/0.6.1-observation-cap at 2026-11-13T03:33:05Z, clean from 2026-11-27T03:33:05Z, 15 restarts in the window. Recomputing from the saved input gave a byte-identical receipt (input sha256 40fe8591bf6c78c0). This shows the check runs on real data, not that the window is clean"
    result: passed
    revision: 1d711eddf2ac335963f51cd802604e839b212a7e
    environment: local-macos-arm64
    content: 552e0adb8469b6e05a2648cdca2cc07d6e9cd0eed92c842f1170d1e653297e27
    observed: 2026-11-13T03:50:48Z
    actor: claude-code (hc t38r)
  - id: 2026-11-13-the-full-test-suite-passed-735-of-735-across-48-targets-cargo
    claim: "The full test suite passed 735 of 735 across 48 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at 383e1ea3, #217's head with main (#248) merged and REQ-cli-062 materialized; the flake-window fixtures passed 24 of 24 and rustfmt --check was clean. No Rust file changed, so clippy was not rerun"
    result: passed
    revision: 383e1ea33c614d1316c2d05b461c05db321778f4
    environment: local-macos-arm64
    content: 16e878e476f860c17e54ab2e82d50841fb624b2308e115d43eb87cdb60c78006
    observed: 2026-11-13T04:54:43Z
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-restarts-ec7983
    claim: "Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: failed (exit 1)."
    result: failed
    command: python3 .github/scripts/test-flake-window.py RestartTests
    revision: decb87e8551e30566ff07465e2c1cfa86287a151
    environment: local-macos-arm64
    content: 7a0326c434e6f3578d5feabd4edfac273f1a1637286afbba5750f9d3f5f5881d
    fingerprint: sha256:7817208cd92f26aba621ff47e37ec640875466d4ba23f4f48a632d6579851375
    case: TC-cli-flake-window-restarts
    case_digest: 666a0615f7ef82a6be52c3d549f5243735eff1462328f4111a0a6f9d29402ecc
    observed: 2026-11-13T05:01:14Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-fails-closed-cbf9e4
    claim: "Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: failed (exit 1)."
    result: failed
    command: python3 .github/scripts/test-flake-window.py FailClosedTests
    revision: decb87e8551e30566ff07465e2c1cfa86287a151
    environment: local-macos-arm64
    content: 7a0326c434e6f3578d5feabd4edfac273f1a1637286afbba5750f9d3f5f5881d
    fingerprint: sha256:b3739b6d72bafbab8af578aeaff262baa42c3e91edb54794b6e8f89078a6d22b
    case: TC-cli-flake-window-fails-closed
    case_digest: 0e0806d15d6b566c3f6d2c5bfe6480ed7feb0f861f604189adc7b8c0b473a2d4
    observed: 2026-11-13T05:01:18Z
    duration_seconds: 1
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-clean-5effb3
    claim: "Ran python3 .github/scripts/test-flake-window.py CleanWindowTests for TC-cli-flake-window-clean: passed (exit 0)."
    result: passed
    command: python3 .github/scripts/test-flake-window.py CleanWindowTests
    revision: 6d69df93c2d29ee38648271b4a18cbf094859e40
    environment: local-macos-arm64
    content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
    fingerprint: sha256:f920ee0e8b42b181363bd792271236b9351c43fda3f29cd63ebd200d677c8eaf
    case: TC-cli-flake-window-clean
    case_digest: 5fb48fb0e12045738962b757c647c98d98cc3e5853ffaefa7966dce77e4024b9
    observed: 2026-11-13T05:02:21Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-restarts-a60440
    claim: "Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: passed (exit 0)."
    result: passed
    command: python3 .github/scripts/test-flake-window.py RestartTests
    revision: 6d69df93c2d29ee38648271b4a18cbf094859e40
    environment: local-macos-arm64
    content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
    fingerprint: sha256:86337cfd8ed176d8d90be3f7c4587b63318e82de083d026a6942ae2da96bba02
    case: TC-cli-flake-window-restarts
    case_digest: 666a0615f7ef82a6be52c3d549f5243735eff1462328f4111a0a6f9d29402ecc
    observed: 2026-11-13T05:02:23Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-blocked-7a226e
    claim: "Ran python3 .github/scripts/test-flake-window.py BlockedTests for TC-cli-flake-window-blocked: passed (exit 0)."
    result: passed
    command: python3 .github/scripts/test-flake-window.py BlockedTests
    revision: 6d69df93c2d29ee38648271b4a18cbf094859e40
    environment: local-macos-arm64
    content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
    fingerprint: sha256:216becf55cbda547302a144f477e2a6af4bc257164d7c3d1b84def3b52118a65
    case: TC-cli-flake-window-blocked
    case_digest: a480bb1749670a6d79b43f09a94931f954195cc93c5a4928953b9518accdb81b
    observed: 2026-11-13T05:02:25Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-fails-closed-fe148d
    claim: "Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: passed (exit 0)."
    result: passed
    command: python3 .github/scripts/test-flake-window.py FailClosedTests
    revision: 6d69df93c2d29ee38648271b4a18cbf094859e40
    environment: local-macos-arm64
    content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
    fingerprint: sha256:0d47e6f94db721901d853ebe7ec8f254757fb68b78bb5b92fff3fb28e1738819
    case: TC-cli-flake-window-fails-closed
    case_digest: 0e0806d15d6b566c3f6d2c5bfe6480ed7feb0f861f604189adc7b8c0b473a2d4
    observed: 2026-11-13T05:02:30Z
    duration_seconds: 2
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-flake-window-wiring-816737
    claim: "Ran python3 .github/scripts/test-flake-window.py WiringTests for TC-cli-flake-window-wiring: passed (exit 0)."
    result: passed
    command: python3 .github/scripts/test-flake-window.py WiringTests
    revision: 6d69df93c2d29ee38648271b4a18cbf094859e40
    environment: local-macos-arm64
    content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
    fingerprint: sha256:2651f01cb66585ce829185f1e4cb01d8394ab91d01098d9fa5ae8b294d12b046
    case: TC-cli-flake-window-wiring
    case_digest: 03d0f18b6f7c3f3416763c407dffaf5a954527710c901efc8c5952455012c242
    observed: 2026-11-13T05:02:32Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-the-full-test-suite-passed-735-of-735-across-48-targets-at
    claim: "The full test suite passed 735 of 735 across 48 targets at 05786a14, #252's head after t384's must-fix, under the heavy-tests lock; the flake-window fixtures passed 27 of 27 and rustfmt --check was clean. No Rust file changed, so clippy was not rerun"
    result: passed
    revision: 05786a14526a1c168e7ff6678d81ac884abd579f
    environment: local-macos-arm64
    content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
    observed: 2026-11-13T05:51:56Z
    actor: claude-code (hc t38r)
  - id: 2026-11-13-live-run-after-the-fix-with-the-flake-labels-on-github-but
    claim: "Live run after the fix, with the flake labels on GitHub but this branch's records not seeded: verdict blocked by #132, #169 and #187 having no bug record naming them (the seed PR #255 adds those) and #247 open with its record on its fix branch; 814 runs, 136 main runs, 4 flake issues, 206 bug records. This shows the check on live data, not that the window is clean"
    result: passed
    revision: 05786a14526a1c168e7ff6678d81ac884abd579f
    environment: local-macos-arm64
    content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
    observed: 2026-11-13T05:51:58Z
    actor: claude-code (hc t38r)
```
