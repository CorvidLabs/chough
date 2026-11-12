---
kind: evidence
id: 2026-11-12-tc-history-story-lists-each-cases-latest-run-c4e085
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:3f08c05b9113ad63aa5c359d1b3eb47ad568e54508dd793058699ec7eb27e722
criteria: [LIFE-7]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:24:16Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact a_story_lists_each_cases_latest_run_once_and_counts_the_earlier_ones for TC-history-story-lists-each-cases-latest-run: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:24:16Z. The raw log stays in the machine-local store.
