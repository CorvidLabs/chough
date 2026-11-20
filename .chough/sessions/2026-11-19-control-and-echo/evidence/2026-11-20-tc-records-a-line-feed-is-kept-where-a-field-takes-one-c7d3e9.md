---
kind: evidence
id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-c7d3e9
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
revision: 26a42a15d428cc7db4805069cc58c0d23330f390
content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
environment: local-macos-arm64
fingerprint: sha256:ccd4edb66094a2cdf925eda72b92ff78abfecc5282678ebcc2377f0f7db9ac57
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:12:04Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26a42a15d428, observed 2026-11-20T04:12:04Z. The raw log stays in the machine-local store.
