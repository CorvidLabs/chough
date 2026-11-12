---
kind: evidence
id: 2026-11-12-tc-cli-intent-gate-summary-names-the-criteria-and-hi-files-5c688e
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
revision: e0131c098169579c13f2498db93514969f149f33
content: f90f0ac78534841119984ed8ae2779eb2acada66c1c412a9c38a7acfd38b6442
environment: local-macos-arm64
fingerprint: sha256:d906ee0c5b7e949a1bab675b9028eb56172086b0ff541cfea28086d1497c3c36
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:02:23Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact an_intent_gate_summary_names_the_criteria_and_their_hi_files for TC-cli-intent-gate-summary-names-the-criteria-and-hi-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e0131c098169, observed 2026-11-12T23:02:23Z. The raw log stays in the machine-local store.
