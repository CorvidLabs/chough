---
kind: evidence
id: 2026-11-13-tc-cli-exercise-rerun-waits-past-the-failing-runs-second-0f1174
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
duration_seconds: 6
revision: ff954002c5fe6f5a0a0834d29d2f6209ace21509
content: 271785d961b582c0ea3d747c220e909a54ca8f430bcbbef337b06f190cdb08a7
environment: local-macos-arm64
fingerprint: sha256:0385bcba39af0e016ea5d78e01f0accbd27c5fe326f8d834bb60488e9792f53c
criteria: [DRIVE-19]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:42:54Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cli -- --exact exercise_rerun_waits_past_the_failing_runs_second_before_verifying for TC-cli-exercise-rerun-waits-past-the-failing-runs-second: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ff954002c5fe, observed 2026-11-13T06:42:54Z. The raw log stays in the machine-local store.
