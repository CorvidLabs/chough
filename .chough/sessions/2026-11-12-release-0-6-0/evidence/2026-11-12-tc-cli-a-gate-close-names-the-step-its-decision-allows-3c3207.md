---
kind: evidence
id: 2026-11-12-tc-cli-a-gate-close-names-the-step-its-decision-allows-3c3207
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact a_gate_close_names_the_step_its_decision_allows_never_chough_next for TC-cli-a-gate-close-names-the-step-its-decision-allows: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-gate-close-names-the-step-its-decision-allows]
case_digest: [d0909d9339d52326c1aa3c2a2c29154bfdf33f369b9d3c65572e2a1a2e85d19b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - a_gate_close_names_the_step_its_decision_allows_never_chough_next
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: cca98758c52eb3f7b42d1f1caf751fc0e5a500c1
content: 94d89ecbe5a0f2d293255d074ccc5b065282faf8c8e453fc4e4ec5f4f093b88f
environment: local-macos-arm64
fingerprint: sha256:30c3826e7c1f278ce479e51d9cb38fc29bb4dd7384d2b78249f23f15b0d81001
criteria: [FLOW-2, FLOW-23]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:03:24Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_gate_close_names_the_step_its_decision_allows_never_chough_next for TC-cli-a-gate-close-names-the-step-its-decision-allows: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cca98758c52e, observed 2026-11-12T21:03:24Z. The raw log stays in the machine-local store.
