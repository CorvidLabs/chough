---
kind: evidence
id: 2026-11-13-tc-cli-exercise-rerun-waits-past-the-failing-runs-second
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact exercise_rerun_waits_past_the_failing_runs_second_before_verifying for TC-cli-exercise-rerun-waits-past-the-failing-runs-second: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-exercise-rerun-waits-past-the-failing-runs-second]
case_digest: [2fcdba62200566d70f3268854ea254ed8f611eb833246f725a34c519545a84f1]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - exercise_rerun_waits_past_the_failing_runs_second_before_verifying
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 190a11aee2a6112a730861c6f84290717326c965
content: 019163cbe0fd37541970689058d86f8044450b785c2b9b7e9ac65d87a6925c86
environment: local-macos-arm64
fingerprint: sha256:938c409cf04220bce5544178c84f98bb08c976f455ac54a017436711588a6129
criteria: [DRIVE-19]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:08:24Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cli -- --exact exercise_rerun_waits_past_the_failing_runs_second_before_verifying for TC-cli-exercise-rerun-waits-past-the-failing-runs-second: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 190a11aee2a6, observed 2026-11-13T06:08:24Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- exercise_rerun_waits_past_the_failing_runs_second_before_verifying stdout ----
- thread 'exercise_rerun_waits_past_the_failing_runs_second_before_verifying' (960906714) panicked at tests/cli.rs:2793:5:
- assertion `left == right` failed: the rerun did not verify the bug:
- TC-shipping-0001: passed, observation 2026-11-13-tc-shipping-0001-a55ef4
- TC-shipping-0002: passed, observation 2026-11-13-tc-shipping-0002
- TC-shipping-0003: passed, observation 2026-11-13-tc-shipping-0003
- Error: verified needs a linked passed observation after the fix, observed after the latest linked failure 2026-11-13-fix-a-reported-bug/2026-11-13-tc-shipping-0001 (observed 2026-11-13T06:08:24Z): link a later pass with `chough bug update 2026-11-13-shipping-charged-at-exactly-the-free-threshold --observation <id>`
-   left: String("open")
-  right: "verified"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 58 filtered out; finished in 2.17s
