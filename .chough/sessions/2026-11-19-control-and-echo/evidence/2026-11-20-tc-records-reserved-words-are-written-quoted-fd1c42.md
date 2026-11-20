---
kind: evidence
id: 2026-11-20-tc-records-reserved-words-are-written-quoted-fd1c42
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
revision: 26a42a15d428cc7db4805069cc58c0d23330f390
content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
environment: local-macos-arm64
fingerprint: sha256:ad171f2b026fb9fbc78193f0b4d8a5f86856fd289953f4ded7b1137d6565fdd0
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:12:21Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26a42a15d428, observed 2026-11-20T04:12:21Z. The raw log stays in the machine-local store.
