---
kind: evidence
id: 2026-11-12-tc-cli-docs-show-the-requirement-evidence-table
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact the_requirement_evidence_table_in_a_changes_testing_md_is_documented for TC-cli-docs-show-the-requirement-evidence-table: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-docs-show-the-requirement-evidence-table]
case_digest: [627d1961f56c1970aaebdfc5fa3382e7b8cf6ec7fb816dbcf8776b44859605fc]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - the_requirement_evidence_table_in_a_changes_testing_md_is_documented
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: a0152789eec8d548d98dbd11f86b0818ad98f494
content: 0ef70fa8fed5b907a0059b2d0f7320b32f187bc45fae18ee147c4deb03536246
environment: local-macos-arm64
fingerprint: sha256:f46f866eabf8d47a49c0a8c72433086a75ab1e6baa87fe94ecd0742400dcab12
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:25:38Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_requirement_evidence_table_in_a_changes_testing_md_is_documented for TC-cli-docs-show-the-requirement-evidence-table: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision a0152789eec8, observed 2026-11-12T20:25:38Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_requirement_evidence_table_in_a_changes_testing_md_is_documented stdout ----
- thread 'the_requirement_evidence_table_in_a_changes_testing_md_is_documented' (922843676) panicked at tests/docs.rs:1692:13:
- the new-project section of docs/cli.md does not show ".specsync/changes/<id>/testing.md"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s
