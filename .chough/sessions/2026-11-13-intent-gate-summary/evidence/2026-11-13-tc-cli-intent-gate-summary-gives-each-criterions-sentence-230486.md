---
kind: evidence
id: 2026-11-13-tc-cli-intent-gate-summary-gives-each-criterions-sentence-230486
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
revision: 4c6e1123c572bbcdd7e4773dac3978c26a38165f
content: 45274f8cb4df179d48551c089c09125f35538fe2807481ec7d30db8d80ab42c6
environment: local-macos-arm64
fingerprint: sha256:eb7cdee34e6aab6e1da24e1ec77150c4086ee127f958a3c4d673bac0c530b4f1
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:39:28Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact an_intent_gate_summary_gives_each_criterions_sentence_and_no_diff_line for TC-cli-intent-gate-summary-gives-each-criterions-sentence: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c6e1123c572, observed 2026-11-13T02:39:28Z. The raw log stays in the machine-local store.
