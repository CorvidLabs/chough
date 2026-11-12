---
kind: evidence
id: 2026-11-12-tc-cli-gate-summary-shows-the-scope-ad7787
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_the_changes_delta_and_plan_first for TC-cli-gate-summary-shows-the-scope: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 0af25ea1480d1d0ce7eadd189f3a8b0c8b520fd2
content: 5511f9a2ec985a55c1f2ae5978bc45fe1b06f0386f884799ca8962145d552f86
environment: local-macos-arm64
fingerprint: sha256:8f4ddc47dc9f2e7a9cede4ff5ee44fa8d10b2209f6a875086372a3099b2b89ea
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:19:21Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_scope_gate_summary_names_the_changes_delta_and_plan_first for TC-cli-gate-summary-shows-the-scope: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0af25ea1480d, observed 2026-11-12T17:19:21Z. The raw log stays in the machine-local store.
