---
kind: evidence
id: 2026-11-20-tc-records-reserved-words-are-written-quoted-c69322
aliases: []
claim: "Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-reserved-words-are-written-quoted]
case_digest: [6c17edd3b466edea721bb9e625c11af9fb75c251abb8c617c7eeaf5ae517ea97]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - checkpoint
  - "--"
  - "--exact"
  - yaml_reserved_words_are_written_quoted_and_read_back_as_text
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 92b89b970299939dd38635eab1c13fb1952b539a
content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
environment: local-macos-arm64
fingerprint: sha256:e7906d8d31a1adf4db269e6b8ed8dac93fd5b25dd56b21d50077b23d4240ef8c
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:23:51Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 92b89b970299, observed 2026-11-20T03:23:51Z. The raw log stays in the machine-local store.
