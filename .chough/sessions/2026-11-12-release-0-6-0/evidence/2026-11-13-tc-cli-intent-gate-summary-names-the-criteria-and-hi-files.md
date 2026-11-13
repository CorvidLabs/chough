---
kind: evidence
id: 2026-11-13-tc-cli-intent-gate-summary-names-the-criteria-and-hi-files
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact an_intent_gate_summary_names_the_criteria_and_their_hi_files for TC-cli-intent-gate-summary-names-the-criteria-and-hi-files: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-intent-gate-summary-names-the-criteria-and-hi-files]
case_digest: [375e7c6fe515b33e0429fe3791fad082f7f9a47425c818912f4881724ec67afa]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - an_intent_gate_summary_names_the_criteria_and_their_hi_files
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:d79f84c60a2a34a7ca01668343975ce42ec2e5ef9abe00c5d076fa833d5ed438
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:23:53Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact an_intent_gate_summary_names_the_criteria_and_their_hi_files for TC-cli-intent-gate-summary-names-the-criteria-and-hi-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:23:53Z. The raw log stays in the machine-local store.
