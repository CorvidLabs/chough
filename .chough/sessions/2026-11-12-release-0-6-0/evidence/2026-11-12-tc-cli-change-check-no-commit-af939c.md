---
kind: evidence
id: 2026-11-12-tc-cli-change-check-no-commit-af939c
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
revision: 91c3824f94a9742f127a480d297fc7f2175e631c
content: c2aa3b745dd7a94452e0b4feee5950b8b356056ae629fdf516c4dcf5c7d1e6fd
environment: local-macos-arm64
fingerprint: sha256:40f76af0b8f76c49ec6bc7db01526c410497edaae4d007fb85cb34e2c66899d8
criteria: [DRIVE-20, DRIVE-21]
requirements: [REQ-cli-057]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:07:05Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact change_check_no_commit_runs_the_plain_check_and_is_never_flagged for TC-cli-change-check-no-commit: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 91c3824f94a9, observed 2026-11-12T21:07:05Z. The raw log stays in the machine-local store.
