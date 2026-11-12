---
kind: evidence
id: 2026-11-12-tc-cli-change-check-no-commit-b7e0ec
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:0999d1cedb0d45f00050e938c7b95d2ad7ab9a4d4fc89c2e162ccffa2286e863
criteria: [DRIVE-20, DRIVE-21]
requirements: [REQ-cli-057]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:23:41Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact change_check_no_commit_runs_the_plain_check_and_is_never_flagged for TC-cli-change-check-no-commit: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:23:41Z. The raw log stays in the machine-local store.
