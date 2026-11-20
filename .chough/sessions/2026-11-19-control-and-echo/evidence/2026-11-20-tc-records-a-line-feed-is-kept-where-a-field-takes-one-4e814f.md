---
kind: evidence
id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-4e814f
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
duration_seconds: 0
revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
environment: local-macos-arm64
fingerprint: sha256:9636125d79d042b66073a29868bed99d0bb5511f2a926c4acc40398f5481159b
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T05:42:01Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 00a1e08cb8f6, observed 2026-11-20T05:42:01Z. The raw log stays in the machine-local store.
