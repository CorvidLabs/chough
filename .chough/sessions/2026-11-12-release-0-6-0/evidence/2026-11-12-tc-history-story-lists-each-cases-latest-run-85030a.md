---
kind: evidence
id: 2026-11-12-tc-history-story-lists-each-cases-latest-run-85030a
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
duration_seconds: 3
revision: 06407b98e650d73a951f505369cb7c694765ffde
content: 20c9710c0555251958526a8d2c4296aeac3f42743b807b3fecd9dd54614bf1da
environment: local-macos-arm64
fingerprint: sha256:1806d4c5b17c4443fab1396c38d4455e50eaa1c4047593941951d0401dc97bc3
criteria: [LIFE-7]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:21:11Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact a_story_lists_each_cases_latest_run_once_and_counts_the_earlier_ones for TC-history-story-lists-each-cases-latest-run: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 06407b98e650, observed 2026-11-12T21:21:11Z. The raw log stays in the machine-local store.
