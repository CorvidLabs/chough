---
kind: evidence
id: 2026-11-18-tc-records-a-fenced-example-and-a-lone-separator-both-save-97319a
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
duration_seconds: 2
revision: 05ea2811ead5c40449265fe3e68f3114375526eb
content: 668614d7d416992d94bdab698001370290f983469c669fc57b014ed26862ee3c
environment: local-macos-arm64
fingerprint: sha256:29c2a63ad4c1dde76e1d3ce357aa412bdfafb8abe3152686b87946a2286330ff
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:06:55Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact a_fenced_example_and_a_lone_separator_both_save for TC-records-a-fenced-example-and-a-lone-separator-both-save: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 05ea2811ead5, observed 2026-11-18T23:06:55Z. The raw log stays in the machine-local store.
