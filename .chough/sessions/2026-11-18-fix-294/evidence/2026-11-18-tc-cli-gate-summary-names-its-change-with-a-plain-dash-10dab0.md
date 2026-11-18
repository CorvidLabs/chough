---
kind: evidence
id: 2026-11-18-tc-cli-gate-summary-names-its-change-with-a-plain-dash-10dab0
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_gate_summary_names_its_change_with_a_plain_dash for TC-cli-gate-summary-names-its-change-with-a-plain-dash: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-summary-names-its-change-with-a-plain-dash]
case_digest: [a19385917de566020aa6f9aa973a87569340ad1af030ac46f8fbf83925a51c74]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_gate_summary_names_its_change_with_a_plain_dash
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: ca8dddce4ba66750a87a5fd8f449ef7a5e233094
content: 1c2ca91075e1ccea7b8b38825e13178c7b8fca4fd6ebbd9095f8bdc439aa191e
environment: local-macos-arm64
fingerprint: sha256:2f18543cd0b90996e6c36a677149f551dfef98d2bf06bc3cfc659e3c62e24582
criteria: [LIFE-21]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T17:09:39Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test gates -- --exact a_gate_summary_names_its_change_with_a_plain_dash for TC-cli-gate-summary-names-its-change-with-a-plain-dash: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ca8dddce4ba6, observed 2026-11-18T17:09:39Z. The raw log stays in the machine-local store.
