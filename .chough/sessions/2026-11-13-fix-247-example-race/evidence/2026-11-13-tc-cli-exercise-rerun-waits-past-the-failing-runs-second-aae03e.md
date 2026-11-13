---
kind: evidence
id: 2026-11-13-tc-cli-exercise-rerun-waits-past-the-failing-runs-second-aae03e
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact exercise_rerun_waits_past_the_failing_runs_second_before_verifying for TC-cli-exercise-rerun-waits-past-the-failing-runs-second: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 8
revision: 72cb5ef655762dcd4b1979e3e1d87149428baa68
content: 471d8569037ea762369eded05e2e6db52624aedf49636a831eb073c7fdc8cdca
environment: local-macos-arm64
fingerprint: sha256:082ca76ae392c6636a05a6f598ac111c3e0b632cc27b51454f146aa427cb3417
criteria: [DRIVE-19]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T07:31:12Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cli -- --exact exercise_rerun_waits_past_the_failing_runs_second_before_verifying for TC-cli-exercise-rerun-waits-past-the-failing-runs-second: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 72cb5ef65576, observed 2026-11-13T07:31:12Z. The raw log stays in the machine-local store.
