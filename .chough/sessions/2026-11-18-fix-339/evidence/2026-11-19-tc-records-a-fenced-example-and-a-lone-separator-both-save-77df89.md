---
kind: evidence
id: 2026-11-19-tc-records-a-fenced-example-and-a-lone-separator-both-save-77df89
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
revision: 08801812d7fe82d42f54451eb25087df5ddaeec6
content: 2b860426c9c1f962e92c5eaef48d4cf1ed31a739e1fc4da29359c8cb5baba200
environment: local-macos-arm64
fingerprint: sha256:f6e721dbc611c0317012eb2aed290817e492d66c68408de4d977dc94a2739386
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:41:18Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact a_fenced_example_and_a_lone_separator_both_save for TC-records-a-fenced-example-and-a-lone-separator-both-save: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 08801812d7fe, observed 2026-11-19T00:41:18Z. The raw log stays in the machine-local store.
