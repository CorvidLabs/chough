---
kind: evidence
id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-b3814b
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
duration_seconds: 1
revision: 92b89b970299939dd38635eab1c13fb1952b539a
content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
environment: local-macos-arm64
fingerprint: sha256:45e888f6dea132359d74feb81c1df1017cdab7d9a1f41795ed09616fd059ff72
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:23:34Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 92b89b970299, observed 2026-11-20T03:23:34Z. The raw log stays in the machine-local store.
