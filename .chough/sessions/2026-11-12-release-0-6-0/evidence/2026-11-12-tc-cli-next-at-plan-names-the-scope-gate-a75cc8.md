---
kind: evidence
id: 2026-11-12-tc-cli-next-at-plan-names-the-scope-gate-a75cc8
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_plan_next_names_the_scope_gate_and_never_specsyncs_approve for TC-cli-next-at-plan-names-the-scope-gate: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-at-plan-names-the-scope-gate]
case_digest: [75777266c77d6f9a658a56e102c54b2e702c75a7001270f5c9dd49664f21d4bd]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - at_plan_next_names_the_scope_gate_and_never_specsyncs_approve
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 0af25ea1480d1d0ce7eadd189f3a8b0c8b520fd2
content: 5511f9a2ec985a55c1f2ae5978bc45fe1b06f0386f884799ca8962145d552f86
environment: local-macos-arm64
fingerprint: sha256:ba1d63488d45565fb13ff73fd48a242e321bf284edbfef7cfee7f4be57badf38
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:19:16Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_plan_next_names_the_scope_gate_and_never_specsyncs_approve for TC-cli-next-at-plan-names-the-scope-gate: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0af25ea1480d, observed 2026-11-12T17:19:16Z. The raw log stays in the machine-local store.
