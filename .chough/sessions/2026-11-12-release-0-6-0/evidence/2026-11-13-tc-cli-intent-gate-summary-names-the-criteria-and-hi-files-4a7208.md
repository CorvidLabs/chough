---
kind: evidence
id: 2026-11-13-tc-cli-intent-gate-summary-names-the-criteria-and-hi-files-4a7208
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:76be87ff99cd2d950f3a7d2ad4393e75d42901bffc4d1ff80dcf29f4af1dbd5e
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:50:35Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact an_intent_gate_summary_names_the_criteria_and_their_hi_files for TC-cli-intent-gate-summary-names-the-criteria-and-hi-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:50:35Z. The raw log stays in the machine-local store.
