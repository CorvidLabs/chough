---
kind: evidence
id: 2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line-22f376
aliases: []
claim: "Ran cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines for TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line]
case_digest: [73ef751d996c495ef5c2c2b09e4780498f345a9ee29cb9e0e0f09a4d3d9c0c5c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - timing
  - "--"
  - "--exact"
  - timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: bc275212d81eb8ac9c8498f189809431ab75340c
content: 289ed18f7eeb791ead6c5c6ddb8da209cbcde473ed6022b7b1aa7c4b4dbb7082
environment: local-macos-arm64
fingerprint: sha256:bbdbe4fbff595dd72e9a1856d94d0ccec190ae8c4c9edeb445584c54b77541ac
criteria: [LIFE-10]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:01:04Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines for TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision bc275212d81e, observed 2026-11-13T03:01:04Z. The raw log stays in the machine-local store.
