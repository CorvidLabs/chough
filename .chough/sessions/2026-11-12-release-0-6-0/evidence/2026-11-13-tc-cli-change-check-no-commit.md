---
kind: evidence
id: 2026-11-13-tc-cli-change-check-no-commit
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
duration_seconds: 2
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:f2250eb12ff128b598f6e9021a259d2baec96401a756189f17526b7fcbcfc7b4
criteria: [DRIVE-20, DRIVE-21]
requirements: [REQ-cli-057]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:20:32Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact change_check_no_commit_runs_the_plain_check_and_is_never_flagged for TC-cli-change-check-no-commit: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:20:32Z. The raw log stays in the machine-local store.
