---
kind: evidence
id: 2026-11-12-tc-cli-docs-show-a-new-project-and-a-delta-a22092
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact a_new_project_path_and_the_delta_format_are_documented for TC-cli-docs-show-a-new-project-and-a-delta: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-docs-show-a-new-project-and-a-delta]
case_digest: [893a075c612c13f8c8075047dd64d476c55bb967f8eded8fd8205aee7c193b9a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - a_new_project_path_and_the_delta_format_are_documented
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 71c823542fe95bc9aa2c6ad1ca3a5eb053175dfe
content: 054b67d651194d55acebf36cc770e6c8aeda5ee26e6c2bf34033671f0eb0f917
environment: local-macos-arm64
fingerprint: sha256:fac3114f25363e119e46bb3ca4b1d7afe45ee80ca75e551a2e79ce7437b47aa5
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:28:01Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact a_new_project_path_and_the_delta_format_are_documented for TC-cli-docs-show-a-new-project-and-a-delta: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 71c823542fe9, observed 2026-11-12T17:28:01Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_new_project_path_and_the_delta_format_are_documented stdout ----
- thread 'a_new_project_path_and_the_delta_format_are_documented' (920590705) panicked at tests/docs.rs:1627:9:
- the new-project section does not show "## MODIFIED"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.00s
