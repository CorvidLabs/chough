---
kind: evidence
id: 2026-11-19-tc-records-a-fenced-example-and-a-lone-separator-both-save
aliases: []
claim: "Ran cargo test --locked --test records -- --exact a_fenced_example_and_a_lone_separator_both_save for TC-records-a-fenced-example-and-a-lone-separator-both-save: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-fenced-example-and-a-lone-separator-both-save]
case_digest: [b0c0eb1fbcfbc846ea3791d3b01dcbe440b4f9bfd06486208eff3500d12c1921]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - records
  - "--"
  - "--exact"
  - a_fenced_example_and_a_lone_separator_both_save
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e72be7de72efed1c9917e9d18329bb4388c3c29a
content: ef0e3be4bc113f74006c5b98c137f39a04a285bdc4e2cec0d9126fc4328b6a36
environment: local-macos-arm64
fingerprint: sha256:eeff6cd28bf6121f277cf41c6b57b8fedb1940c16ea066f5637a788ffe58a679
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:21:35Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact a_fenced_example_and_a_lone_separator_both_save for TC-records-a-fenced-example-and-a-lone-separator-both-save: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e72be7de72ef, observed 2026-11-19T00:21:35Z. The raw log stays in the machine-local store.
