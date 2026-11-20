---
kind: evidence
id: 2026-11-20-tc-cli-g2-evidence-with-no-case-counts-a722d2
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-g2-evidence-with-no-case-counts]
case_digest: [951df83095a2697734c253fa89bb31caa1a5ed68454d932a225584d2218ac821]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - evidence_with_no_case_counts_in_the_tests_line
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
environment: local-macos-arm64
fingerprint: sha256:52cb696a0efd0a58231d3fd6430c2148af7d9d4f54d1fabd447b0bbd353dd83f
criteria: [DRIVE-24]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T21:09:38Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 01593390f15c, observed 2026-11-20T21:09:38Z. The raw log stays in the machine-local store.
