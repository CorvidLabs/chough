---
kind: evidence
id: 2026-11-13-tc-reporting-timing-says-elapsed-wall-clock-once-on-every-line
aliases: []
claim: "Ran cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines for TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: 1a57d6f323ee083aac99cc100762d0169f2a7c2a
content: 4a0504b923b86eb60de2e722b0fa05dcd4ce59cdea6d267a4415927f86ca2f33
environment: local-macos-arm64
fingerprint: sha256:8ad107876f1fe089476924b718b9b9c9696ea3751ded7ce3b4846694f29d3ac5
criteria: [LIFE-10]
requirements: [REQ-reporting-001]
contracts: [specs/reporting/reporting.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:56:57Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test timing -- --exact timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines for TC-reporting-timing-says-elapsed-wall-clock-once-on-every-line: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 1a57d6f323ee, observed 2026-11-13T02:56:57Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines stdout ----
- thread 'timing_says_elapsed_wall_clock_once_on_gate_run_and_fix_lines' (932272121) panicked at tests/timing.rs:2582:5:
- assertion `left == right` failed: # Chough timing
- Every duration is elapsed wall-clock time, not effort. A time no record holds is unknown, never zero. Estimates come from recorded history and are not targets.
- Generated 2026-11-13T02:56:56Z.
- ## Stages
- Before the first record:
- - intent, iteration 1: unknown (no transition before the first record) -> 2023-11-14T22:13:20Z, unknown (no transition before the first record); waiting on a human unknown (no transition before the first record); active unknown (no transition before the first record)
- ### Cycle 1 (started 2023-11-14T22:13:20Z, completed)
- - design, iteration 1: 2023-11-14T22:13:20Z -> 2023-11-14T22:15:00Z (entered by advance), 1m 40s; waiting on a human 0s; active 1m 40s
- - plan, iteration 1: 2023-11-14T22:15:00Z -> 2023-11-14T22:18:20Z (entered by skip), 3m 20s; waiting on a human 0s; active 3m 20s
- - build, iteration 1: 2023-11-14T22:18:20Z -> 2023-11-14T22:20:00Z (entered by advance), 1m 40s; waiting on a human 50s; active 50s
- - build, iteration 2: 2023-11-14T22:20:00Z -> 2023-11-14T22:23:20Z (entered by iterate), 3m 20s; waiting on a human 50s; active 2m 30s
- - build paused: 2023-11-14T22:23:20Z -> 2023-11-14T22:28:20Z (entered by pause), 5m 0s
- - build, iteration 2: 2023-11-14T22:28:20Z -> 2023-11-14T22:30:00Z (entered by resume), 1m 40s; waiting on a human 0s; active 1m 40s
- - bugs (side stage): 2023-11-14T22:30:00Z -> 2023-11-14T22:31:40Z (entered by enter), 1m 40s; waiting on a human 0s; active 1m 40s
- - build, iteration 1: 2023-11-14T22:31:40Z -> 2023-11-14T22:33:20Z (entered by return), 1m 40s; waiting on a human 0s; active 1m 40s
- - plan, iteration 1: 2023-11-14T22:33:20Z -> 2023-11-14T22:34:10Z (entered by back), 50s; waiting on a human 0s; active 50s
- - build, it
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.59s
