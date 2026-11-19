---
kind: evidence
id: 2026-11-19-tc-records-a-fenced-example-and-a-lone-separator-both-save-96b8af
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
revision: 4f74fa25cd491a82518ff5aead551e4814f3e043
content: 9f356f9326eedc20a0cb6c07e4dee6e014efc412386eb573f09bdec9a54197b6
environment: local-macos-arm64
fingerprint: sha256:2a02d36edac544e06581da80d70749214e4db8f96af3853074329fb2e300b8a2
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T01:08:08Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact a_fenced_example_and_a_lone_separator_both_save for TC-records-a-fenced-example-and-a-lone-separator-both-save: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4f74fa25cd49, observed 2026-11-19T01:08:08Z. The raw log stays in the machine-local store.
