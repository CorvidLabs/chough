---
kind: evidence
id: 2026-11-18-tc-records-a-fenced-example-and-a-lone-separator-both-save
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
content: 97e0223af6f3dd0d6274c5f2ce6b1aaf50c514d9e6f2573e80abd3b56d5ed8db
environment: local-macos-arm64
fingerprint: sha256:fbcc0d90d39d5ec07ca8cf0b6fdd85265c9b2e552c17307bceafaf527b5c136a
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T21:32:10Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact a_fenced_example_and_a_lone_separator_both_save for TC-records-a-fenced-example-and-a-lone-separator-both-save: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision af8ca8901b2a, observed 2026-11-18T21:32:10Z. The raw log stays in the machine-local store.
