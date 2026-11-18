---
kind: evidence
id: 2026-11-18-tc-records-a-fenced-example-and-a-lone-separator-both-save-ed6ca5
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
revision: af8ca8901b2a23affcd40ac74865d99637448346
content: 55c3fda98068cdcd631bca47109a154d0102300f8845de147a18d4d48de96e4a
environment: local-macos-arm64
fingerprint: sha256:32e319f3a24b011d26610fb2976d0c398c72add994cdd6794fd936f47d8108fa
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T21:33:06Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact a_fenced_example_and_a_lone_separator_both_save for TC-records-a-fenced-example-and-a-lone-separator-both-save: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision af8ca8901b2a, observed 2026-11-18T21:33:06Z. The raw log stays in the machine-local store.
