---
kind: evidence
id: 2026-11-13-tc-cli-ci-runs-daily-with-a-real-archive-base-c0b0ac
aliases: []
claim: "Ran cargo test --locked --test package -- --exact ci_runs_once_a_day_on_main_with_a_real_archive_base for TC-cli-ci-runs-daily-with-a-real-archive-base: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 42eb427cade09717dfaca3cf204b2c34f87eaf83
content: 5cddd4b1ad7c03fb02e5dc94168d1d37865bbb44836219b752e10f79888af6ee
environment: local-macos-arm64
fingerprint: sha256:cfe0fa2fc62b46f934810d8cf890ea38fcc03842c6269e07e7e298a28b5fc822
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-064]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T05:48:19Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test package -- --exact ci_runs_once_a_day_on_main_with_a_real_archive_base for TC-cli-ci-runs-daily-with-a-real-archive-base: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 42eb427cade0, observed 2026-11-13T05:48:19Z. The raw log stays in the machine-local store.
