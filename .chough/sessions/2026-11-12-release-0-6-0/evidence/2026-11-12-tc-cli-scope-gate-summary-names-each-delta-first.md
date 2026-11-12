---
kind: evidence
id: 2026-11-12-tc-cli-scope-gate-summary-names-each-delta-first
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_each_delta_first_with_its_requirements for TC-cli-scope-gate-summary-names-each-delta-first: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-scope-gate-summary-names-each-delta-first]
case_digest: [4a2d1c45218f0e0dfb72c5e4f6488080990be96638ee6943e2b77d4fdb233e4b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_scope_gate_summary_names_each_delta_first_with_its_requirements
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 7fb91195b8f27c97248de42544fd07a89ccc4dbd
content: dcb4f087dd1af97765710e6f3754c942490fbfd0fd5d504ff31fe35cbbd50233
environment: local-macos-arm64
fingerprint: sha256:5194a9ef265eb1366d8224cf3085e07395bdbdd5b0740fea1a4f0ba01ff8eed2
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:56:04Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_each_delta_first_with_its_requirements for TC-cli-scope-gate-summary-names-each-delta-first: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 7fb91195b8f2, observed 2026-11-12T22:56:04Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_scope_gate_summary_names_each_delta_first_with_its_requirements stdout ----
- thread 'a_scope_gate_summary_names_each_delta_first_with_its_requirements' (927169478) panicked at tests/gates.rs:736:5:
- Gate: Scope approval of g1
- Change: g1 — Free shipping at the threshold (approved)
- What changed: 8 files, +284 −1 since e9529570
-   .specsync/changes/g1/change.md +42 −0
-   .specsync/changes/g1/context.md +42 −0
-   .specsync/changes/g1/plan.md +42 −0
-   .specsync/changes/g1/tasks.md +42 −0
-   .specsync/changes/g1/testing.md +42 −0
- Tests: 1 passed, 1 failed, 0 not run
-   Failing: TC-cli-g1-fail
- Risks: none recorded
- Diff: git diff e9529570a08b397c21d9d0c463026ac5c1810029..737485531d75788a02af846003e970ca73b1d185
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out; finished in 1.42s
