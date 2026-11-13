---
kind: evidence
id: 2026-11-13-tc-history-story-receipts-last-and-bugs-named
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_story_puts_its_receipts_last_and_names_its_bugs for TC-history-story-receipts-last-and-bugs-named: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-story-receipts-last-and-bugs-named]
case_digest: [bcd5cb75ed64eb357408b3541926b37728275d7998b0c7a4fb696fbc519cfba7]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_story_puts_its_receipts_last_and_names_its_bugs
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:a76d0d35ae32b4f1cc72172f926afda8494d2c9dbeebe969788fa9143ca09d5a
criteria: [LIFE-7, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:24:52Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact a_story_puts_its_receipts_last_and_names_its_bugs for TC-history-story-receipts-last-and-bugs-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:24:52Z. The raw log stays in the machine-local store.
