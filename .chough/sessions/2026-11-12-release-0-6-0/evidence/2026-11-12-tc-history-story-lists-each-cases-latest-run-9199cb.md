---
kind: evidence
id: 2026-11-12-tc-history-story-lists-each-cases-latest-run-9199cb
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
revision: 91c3824f94a9742f127a480d297fc7f2175e631c
content: c2aa3b745dd7a94452e0b4feee5950b8b356056ae629fdf516c4dcf5c7d1e6fd
environment: local-macos-arm64
fingerprint: sha256:a28634a876ff395e07b4f61fddca5868fc43871985bd5348998638680615d3e0
criteria: [LIFE-7]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:07:29Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact a_story_lists_each_cases_latest_run_once_and_counts_the_earlier_ones for TC-history-story-lists-each-cases-latest-run: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 91c3824f94a9, observed 2026-11-12T21:07:29Z. The raw log stays in the machine-local store.
