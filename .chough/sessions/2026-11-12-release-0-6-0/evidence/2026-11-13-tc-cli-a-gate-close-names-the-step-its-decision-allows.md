---
kind: evidence
id: 2026-11-13-tc-cli-a-gate-close-names-the-step-its-decision-allows
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
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:01365ed8ee6e5333d2277edf179379dfab7530d0ea000f9548cd16246149419d
criteria: [FLOW-2, FLOW-23]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:19:43Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_gate_close_names_the_step_its_decision_allows_never_chough_next for TC-cli-a-gate-close-names-the-step-its-decision-allows: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:19:43Z. The raw log stays in the machine-local store.
