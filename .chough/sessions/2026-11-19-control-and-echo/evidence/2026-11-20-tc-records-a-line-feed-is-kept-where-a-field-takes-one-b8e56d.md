---
kind: evidence
id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-b8e56d
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
duration_seconds: 2
revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
environment: local-macos-arm64
fingerprint: sha256:559a78dd4c4198517645117a9fe9eb26b313d0518333de9181d9952c4e538cc8
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:12:01Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1406385069c2, observed 2026-11-20T06:12:01Z. The raw log stays in the machine-local store.
