---
kind: evidence
id: 2026-11-13-tc-cli-change-check-no-commit-350a48
aliases: []
claim: "Ran cargo test --locked --test direct_writes -- --exact change_check_no_commit_runs_the_plain_check_and_is_never_flagged for TC-cli-change-check-no-commit: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-check-no-commit]
case_digest: [e21362126f90d4909840923c5a662c530035031a40d8486a07c9e59ba94dcdec]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - direct_writes
  - "--"
  - "--exact"
  - change_check_no_commit_runs_the_plain_check_and_is_never_flagged
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:ad35c44973b11bccef02b3cb8783decb675452ad1b8147607d179e8cdc540663
criteria: [DRIVE-20, DRIVE-21]
requirements: [REQ-cli-057]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:12:49Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact change_check_no_commit_runs_the_plain_check_and_is_never_flagged for TC-cli-change-check-no-commit: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:12:49Z. The raw log stays in the machine-local store.
