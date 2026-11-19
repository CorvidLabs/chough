---
kind: evidence
id: 2026-11-19-tc-history-a-storys-verified-line-shows-the-entrys-limitations-9142df
aliases: []
claim: "Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-storys-verified-line-shows-the-entrys-limitations: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-storys-verified-line-shows-the-entrys-limitations]
case_digest: [18a398b09c6239b8e00a8f462c48175d172acebc4dd3d749a5d29f92d4fdcfc9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - evidence
  - "--"
  - "--exact"
  - a_story_receipt_carries_each_field_its_entry_knows
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: eaaab98b0aeb0966a51d4c439ac61247dd4462b1
content: dd9af3c0f877ee390378b1ce8b28d32df9c8451f0a41a718c6b48df63a549578
environment: local-macos-arm64
fingerprint: sha256:2a6fc99f10704fd1240535f68bfdfeb389b9e809f3ae25b1f76238e202114aa6
criteria: [KNOW-35]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T01:15:06Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-storys-verified-line-shows-the-entrys-limitations: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision eaaab98b0aeb, observed 2026-11-19T01:15:06Z. The raw log stays in the machine-local store.
