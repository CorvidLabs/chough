---
kind: evidence
id: 2026-11-13-tc-cli-intent-gate-summary-gives-each-criterions-sentence-8efa5b
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact an_intent_gate_summary_gives_each_criterions_sentence_and_no_diff_line for TC-cli-intent-gate-summary-gives-each-criterions-sentence: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-intent-gate-summary-gives-each-criterions-sentence]
case_digest: [ee99a08d2f8ab309c7f29f482e92c61b03157bce9576825f37a1a4f6a3ef0dd8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - an_intent_gate_summary_gives_each_criterions_sentence_and_no_diff_line
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 8e0feedecb1cbe101c723067a5c300d2cbad07fc
content: a4cfc78f0189db0c9bae41beff3e0201fb02123516fe566e98c9dd7f65c48295
environment: local-macos-arm64
fingerprint: sha256:bd44b9736583792497226010c49d49705373f280c6f7558e0a12e82b01377399
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:34:06Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact an_intent_gate_summary_gives_each_criterions_sentence_and_no_diff_line for TC-cli-intent-gate-summary-gives-each-criterions-sentence: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8e0feedecb1c, observed 2026-11-13T02:34:06Z. The raw log stays in the machine-local store.
