---
kind: evidence
id: 2026-11-12-tc-cli-failed-run-body-says-why
aliases: []
claim: "Ran cargo test --locked --test cases -- --exact a_failed_runs_body_says_the_expected_output_was_missing_before_its_key_lines for TC-cli-failed-run-body-says-why: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-failed-run-body-says-why]
case_digest: [149a7086b776e5da4e4d15b857fedbf7ff5317fbaa777ca230bc76ae4e701069]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cases
  - "--"
  - "--exact"
  - a_failed_runs_body_says_the_expected_output_was_missing_before_its_key_lines
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 4e245263fe4e35b757d9b0531a0992c7cf0f884d
content: 073d7330cf258fdd25b33d439e35f9344842f8fdfa2a1cd81c388ab4af69e3ca
environment: local-macos-arm64
fingerprint: sha256:a4647a71adbdabb87a89509d84bfca8565e560f8e11b6b3977c98687099216b7
criteria: [LIFE-7]
requirements: [REQ-cli-017]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:04:13Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact a_failed_runs_body_says_the_expected_output_was_missing_before_its_key_lines for TC-cli-failed-run-body-says-why: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 4e245263fe4e, observed 2026-11-12T23:04:13Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_failed_runs_body_says_the_expected_output_was_missing_before_its_key_lines stdout ----
- thread 'a_failed_runs_body_says_the_expected_output_was_missing_before_its_key_lines' (927438621) panicked at tests/cases.rs:2963:28:
- the body does not say why it failed:
- **Ran sh -c echo 2 for TC-cli-0001: failed (exit 0).**
- Recorded as failed in local at revision de6983a69ecd, observed 2026-11-12T23:04:12Z. The raw log stays in the machine-local store.
- ## Key failing lines
- - 2
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 37 filtered out; finished in 1.36s
