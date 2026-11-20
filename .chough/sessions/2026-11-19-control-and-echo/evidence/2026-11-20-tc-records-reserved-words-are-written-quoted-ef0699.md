---
kind: evidence
id: 2026-11-20-tc-records-reserved-words-are-written-quoted-ef0699
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
revision: 17f214884b36a9edd288d49c998da9712d7c4117
content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
environment: local-macos-arm64
fingerprint: sha256:4c9f3150f5231a234e4e0bd5674e6286ee722d0bd3c0be52a09f0febbbbd632c
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:26:29Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17f214884b36, observed 2026-11-20T06:26:29Z. The raw log stays in the machine-local store.
