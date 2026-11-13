---
kind: evidence
id: 2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line-24eb81
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
revision: 4001748c5694f643883219e37d690c4271bfc064
content: 489d14fd5c396a13d756cc137bef855dfad477971358d696dc797684d4e06219
environment: local-macos-arm64
fingerprint: sha256:ffabe3eb00f8c4f28eec9dbc9d7d61228048d32aad6c52b69220d771fecec013
criteria: [LIFE-10]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:31:03Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines for TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4001748c5694, observed 2026-11-13T03:31:03Z. The raw log stays in the machine-local store.
