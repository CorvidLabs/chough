---
kind: evidence
id: 2026-11-20-tc-records-reserved-words-are-written-quoted
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
duration_seconds: 0
revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
environment: local-macos-arm64
fingerprint: sha256:6dc42844d959add1ae6716378c04dc2260fd1ce8301412321b6420e79620c15a
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:21:30Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d8aa8a63acd5, observed 2026-11-20T03:21:30Z. The raw log stays in the machine-local store.
