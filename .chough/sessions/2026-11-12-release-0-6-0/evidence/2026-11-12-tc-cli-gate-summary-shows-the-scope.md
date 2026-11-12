---
kind: evidence
id: 2026-11-12-tc-cli-gate-summary-shows-the-scope
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_the_changes_delta_and_plan_first for TC-cli-gate-summary-shows-the-scope: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-summary-shows-the-scope]
case_digest: [9fd29edee0242a72f492822ac722ba75634edd4a3fa2e73596d01b12d6f88c84]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_scope_gate_summary_names_the_changes_delta_and_plan_first
exit_code: 101
tests_run: 1
duration_seconds: 11
revision: da98f439e0a8db2a2c9cde127a83426d96730f49
content: 936e291c15b0e4e6a4c5e2b1f42d6ba1a17ecca8018baad29bbbaba7196270f8
environment: local-macos-arm64
fingerprint: sha256:2647a8f59ffd7855e8ae1548d78b1190872a0730e9daa86c9e36c50ff6d60c25
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:47:45Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_the_changes_delta_and_plan_first for TC-cli-gate-summary-shows-the-scope: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision da98f439e0a8, observed 2026-11-12T16:47:45Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_scope_gate_summary_names_the_changes_delta_and_plan_first stdout ----
- thread 'a_scope_gate_summary_names_the_changes_delta_and_plan_first' (919489324) panicked at tests/gates.rs:608:9:
- the change's own files come first: Gate: Scope approval of g1
- Change: g1 — Free shipping at the threshold (approved)
- What changed: 1 files, +60 −1 since 5d61f6a6
-   src/input.txt +60 −1
- Tests: 1 passed, 1 failed, 0 not run
-   Failing: TC-cli-g1-fail
- Risks:
-   - A rounding edge at exactly fifty dollars.
- Diff: git diff 5d61f6a641808020836c07f226cc7e5ec2b38729..113d1bbe95a6e06124cf404af6e87375e3faad68
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 9 filtered out; finished in 8.27s
