---
kind: evidence
id: 2026-11-13-tc-cli-a-gate-close-names-the-step-its-decision-allows-d189f8
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:a66e95c0d4f01c255fead20b6bd48b7af560da8d7b1ca5e4ac155ec5cad51e73
criteria: [FLOW-2, FLOW-23]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:11:34Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_gate_close_names_the_step_its_decision_allows_never_chough_next for TC-cli-a-gate-close-names-the-step-its-decision-allows: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:11:34Z. The raw log stays in the machine-local store.
