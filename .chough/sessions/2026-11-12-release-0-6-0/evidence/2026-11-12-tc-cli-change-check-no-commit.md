---
kind: evidence
id: 2026-11-12-tc-cli-change-check-no-commit
aliases: []
claim: "Ran cargo test --locked --test direct_writes -- --exact change_check_no_commit_runs_the_plain_check_and_is_never_flagged for TC-cli-change-check-no-commit: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 804bdb506371f0f50b97e76f0c09cc33daf439af
content: 431b305478c9131e54c07bc6bd895e4d5380b9af25bb28dad9e6056aed6a4f09
environment: local-macos-arm64
fingerprint: sha256:43057fa4fbe7b0864ebeadef8ed1cb0cc5f369a6770dd7a299b9e24e5599ab03
criteria: [DRIVE-20, DRIVE-21]
requirements: [REQ-cli-057]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:05:36Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact change_check_no_commit_runs_the_plain_check_and_is_never_flagged for TC-cli-change-check-no-commit: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 804bdb506371, observed 2026-11-12T21:05:36Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- change_check_no_commit_runs_the_plain_check_and_is_never_flagged stdout ----
- thread 'change_check_no_commit_runs_the_plain_check_and_is_never_flagged' (924233654) panicked at tests/direct_writes.rs:428:5:
- error: unexpected argument '--no-commit' found
-   tip: to pass '--no-commit' as a value, use '-- --no-commit'
- Usage: chough change check <ID>
- For more information, try '--help'.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10 filtered out; finished in 1.22s
