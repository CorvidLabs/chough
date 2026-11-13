---
kind: evidence
id: 2026-11-13-tc-history-story-lists-each-cases-latest-run
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_story_lists_each_cases_latest_run_once_and_counts_the_earlier_ones for TC-history-story-lists-each-cases-latest-run: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-story-lists-each-cases-latest-run]
case_digest: [ef1839f327d69096d9d531b15028dde215fff68b8319ba0b3aa973441903d266]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_story_lists_each_cases_latest_run_once_and_counts_the_earlier_ones
exit_code: 0
tests_run: 1
duration_seconds: 4
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:2eb29f93d6acb8ca0f2c04b4e1d3d4a20df27b5612bc5a6962636871dd6bf880
criteria: [LIFE-7]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:21:18Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact a_story_lists_each_cases_latest_run_once_and_counts_the_earlier_ones for TC-history-story-lists-each-cases-latest-run: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:21:18Z. The raw log stays in the machine-local store.
