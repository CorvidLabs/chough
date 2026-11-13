---
kind: evidence
id: 2026-11-13-tc-cli-scope-gate-summary-says-adds-and-modifies
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_says_what_each_delta_adds_and_what_it_modifies for TC-cli-scope-gate-summary-says-adds-and-modifies: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-scope-gate-summary-says-adds-and-modifies]
case_digest: [d377a0edb3b198fede5a257e3ee6f64658054dbf22a66b9584fa41d8dfd2948f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_scope_gate_summary_says_what_each_delta_adds_and_what_it_modifies
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 0f34dbe2b3c86c306d2f7dc95102c54115053209
content: d24d7aa7431e82f0b13decfced5489c9b5dfe4a80a7080a83460ee90036ba7ee
environment: local-macos-arm64
fingerprint: sha256:d66b5c771ffb4ad885be8ee596f037f8d3148555e42cce8e3beff6c70913103b
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:25:19Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_says_what_each_delta_adds_and_what_it_modifies for TC-cli-scope-gate-summary-says-adds-and-modifies: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 0f34dbe2b3c8, observed 2026-11-13T02:25:19Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_scope_gate_summary_says_what_each_delta_adds_and_what_it_modifies stdout ----
- thread 'a_scope_gate_summary_says_what_each_delta_adds_and_what_it_modifies' (930819985) panicked at tests/gates.rs:941:5:
- Gate: Scope approval of g1
- Change: g1 — Free shipping at the threshold (approved)
- What changed: 4 files, +93 −1 since 50673094
-   .specsync/changes/g1/deltas/history.md +15 −0 (adds or modifies REQ-history-002, REQ-history-003)
-   .specsync/changes/g1/deltas/cli.md +5 −0 (adds or modifies REQ-cli-001)
-   .specsync/changes/g1/plan.md +13 −0
-   src/input.txt +60 −1
- Tests: 1 passed, 1 failed, 0 not run
-   Failing: TC-cli-g1-fail
- Risks:
-   - A rounding edge at exactly fifty dollars.
- Diff: git diff 5067309401441c30e4e36d363463215cfeb0fd9f..2bc4f09fc746085e8840f8c4b03c92d0268b8561
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 1.62s
