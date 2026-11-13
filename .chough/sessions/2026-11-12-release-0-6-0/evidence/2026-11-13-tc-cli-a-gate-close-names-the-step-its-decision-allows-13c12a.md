---
kind: evidence
id: 2026-11-13-tc-cli-a-gate-close-names-the-step-its-decision-allows-13c12a
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:4101279f93935b80982a202ec0fe7b29adfb4b3d4620fa5cb9931291971a8d71
criteria: [FLOW-2, FLOW-23]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:45:17Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_gate_close_names_the_step_its_decision_allows_never_chough_next for TC-cli-a-gate-close-names-the-step-its-decision-allows: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:45:17Z. The raw log stays in the machine-local store.
