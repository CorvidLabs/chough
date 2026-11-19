---
kind: evidence
id: 2026-11-19-tc-history-a-storys-verified-line-shows-the-entrys-limitations
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
duration_seconds: 0
revision: 880cc9e641c6f25bebf283acfa2f6c4e68574e40
content: 2690d0cdd258920ae72e12ce877f189380a8def755f161a72fa7e4f07ecf65bf
environment: local-macos-arm64
fingerprint: sha256:691783e8e797645a0fff852c1213693e129c433872b44bc3696edfb4c0aa7b32
criteria: [KNOW-35]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:23:09Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-storys-verified-line-shows-the-entrys-limitations: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 880cc9e641c6, observed 2026-11-19T00:23:09Z. The raw log stays in the machine-local store.
