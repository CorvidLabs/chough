---
kind: evidence
id: 2026-11-13-tc-cli-ci-runs-daily-with-a-real-archive-base-d92603
aliases: []
claim: "Ran cargo test --locked --test package -- --exact ci_runs_once_a_day_on_main_with_a_real_archive_base for TC-cli-ci-runs-daily-with-a-real-archive-base: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ci-runs-daily-with-a-real-archive-base]
case_digest: [15d2048e43a7b94c7360c05ac0db11dda31c5b1a3fbcddc21ffb8ff22c99c0af]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - package
  - "--"
  - "--exact"
  - ci_runs_once_a_day_on_main_with_a_real_archive_base
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: e49e45451bef668d453a28a627b228d2a0ed0cc3
content: 5e1f0ad8dd25063af34658fbb0918c2f501dfcf81e56ead90e4b24e52ca9db50
environment: local-macos-arm64
fingerprint: sha256:1a0714de491e10ea3bba0458a8372a883883f60c01b93a27f29e474bb5a824ac
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-064]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T05:46:31Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test package -- --exact ci_runs_once_a_day_on_main_with_a_real_archive_base for TC-cli-ci-runs-daily-with-a-real-archive-base: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision e49e45451bef, observed 2026-11-13T05:46:31Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- ci_runs_once_a_day_on_main_with_a_real_archive_base stdout ----
- thread 'ci_runs_once_a_day_on_main_with_a_real_archive_base' (956881708) panicked at tests/package.rs:4159:9:
- ci.yml's `on:` block lacks `schedule:`: [
-     "  push:",
-     "    branches: [main]",
-     "  # Every pull request, including stacked ones whose base is not main.",
-     "  pull_request:",
- ]
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s
