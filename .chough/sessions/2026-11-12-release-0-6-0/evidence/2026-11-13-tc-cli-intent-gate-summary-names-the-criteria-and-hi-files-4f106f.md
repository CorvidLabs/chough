---
kind: evidence
id: 2026-11-13-tc-cli-intent-gate-summary-names-the-criteria-and-hi-files-4f106f
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:285a3f107c520ff75a26972f98fdceeffce26c9c4ee0a3b7065a7965e26102f4
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:18:07Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact an_intent_gate_summary_names_the_criteria_and_their_hi_files for TC-cli-intent-gate-summary-names-the-criteria-and-hi-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:18:07Z. The raw log stays in the machine-local store.
