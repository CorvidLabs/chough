---
kind: evidence
id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-b60c5d
aliases: []
claim: "Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-line-feed-is-kept-where-a-field-takes-one]
case_digest: [45ed1184127f3ca3620c7d537003d97b60481a512e736cf5f63c7964aa4a70b4]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - output_safety
  - "--"
  - "--exact"
  - a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
environment: local-macos-arm64
fingerprint: sha256:b9d44f9a73943893d4df8fd976e8f8f5e16713da388a32e6e685faa5ca4f0f3b
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:38:34Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6816b8fe5e23, observed 2026-11-20T06:38:34Z. The raw log stays in the machine-local store.
