---
kind: evidence
id: 2026-11-12-tc-cli-scope-gate-summary-counts-uncommitted-change-files
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_counts_uncommitted_and_untracked_change_files for TC-cli-scope-gate-summary-counts-uncommitted-change-files: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-scope-gate-summary-counts-uncommitted-change-files]
case_digest: [1718cddf6a47a27f123ce01376d2fed1c6c9fbc9475afbbc796b47ad52c01c07]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_scope_gate_summary_counts_uncommitted_and_untracked_change_files
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 730aba9f2157cb64d9df9c89788b5df975ad6e9d
content: 1c4f407c8784eb8b298a8337feb1bf3ce8a34c5bd3774e002b037f307e191ac7
environment: local-macos-arm64
fingerprint: sha256:b22d415fe9d4940dcc2cd9bbd2a58a29c731bf46cde7a565180b85690efe07e2
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:48:20Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_counts_uncommitted_and_untracked_change_files for TC-cli-scope-gate-summary-counts-uncommitted-change-files: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 730aba9f2157, observed 2026-11-12T23:48:20Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_scope_gate_summary_counts_uncommitted_and_untracked_change_files stdout ----
- thread 'a_scope_gate_summary_counts_uncommitted_and_untracked_change_files' (928383231) panicked at tests/gates.rs:864:5:
- assertion `left == right` failed: Gate: Scope approval of g1
- Change: g1 — Free shipping at the threshold (approved)
- What changed: 0 files, +0 −0 since 73f0935b
- Tests: 1 passed, 1 failed, 0 not run
-   Failing: TC-cli-g1-fail
- Risks:
-   - A rounding edge at exactly fifty dollars.
- Diff: git diff 73f0935ba8e11c815fae49ee92b2df12c383420e..73f0935ba8e11c815fae49ee92b2df12c383420e
-   left: "What changed: 0 files, +0 −0"
-  right: "What changed: 4 files, +14 −0"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 13 filtered out; finished in 1.65s
