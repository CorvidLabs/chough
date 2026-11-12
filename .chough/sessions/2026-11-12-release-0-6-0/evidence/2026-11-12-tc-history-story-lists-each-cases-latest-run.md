---
kind: evidence
id: 2026-11-12-tc-history-story-lists-each-cases-latest-run
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_story_lists_each_cases_latest_run_once_and_counts_the_earlier_ones for TC-history-story-lists-each-cases-latest-run: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 5
revision: 804bdb506371f0f50b97e76f0c09cc33daf439af
content: 431b305478c9131e54c07bc6bd895e4d5380b9af25bb28dad9e6056aed6a4f09
environment: local-macos-arm64
fingerprint: sha256:ecea7a108d4f168b09efedfeebafffa1965e9cba0c1087c6da24a5d3f1ebac0f
criteria: [LIFE-7]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:05:59Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact a_story_lists_each_cases_latest_run_once_and_counts_the_earlier_ones for TC-history-story-lists-each-cases-latest-run: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 804bdb506371, observed 2026-11-12T21:05:59Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_story_lists_each_cases_latest_run_once_and_counts_the_earlier_ones stdout ----
- thread 'a_story_lists_each_cases_latest_run_once_and_counts_the_earlier_ones' (924235580) panicked at tests/sessions.rs:572:5:
- assertion `left == right` failed: the command is repeated: - Ran sh -c echo ok for TC-cli-0001: passed (exit 0). — `sh -c echo ok` edc35ca1792e ^2026-11-12-tc-cli-0001-64d065
-   left: 2
-  right: 1
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 29 filtered out; finished in 4.02s
