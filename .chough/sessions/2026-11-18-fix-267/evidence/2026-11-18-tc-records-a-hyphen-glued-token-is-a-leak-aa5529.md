---
kind: evidence
id: 2026-11-18-tc-records-a-hyphen-glued-token-is-a-leak-aa5529
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact a_hyphen_glued_token_is_refused_at_the_write_and_found_by_the_doctor for TC-records-a-hyphen-glued-token-is-a-leak: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-hyphen-glued-token-is-a-leak]
case_digest: [16f95e5c92c506145243f581041fca88fb5cccd573b19f70dd1b0911b5def849]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - a_hyphen_glued_token_is_refused_at_the_write_and_found_by_the_doctor
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 4a295f06742cefd4dd5756a1870a80e40f76f3bf
content: 82d8f33bcb983d0394968578f716222a5a5c279e4d9ebef1654e57d6cad26611
environment: local-macos-arm64
fingerprint: sha256:282ccd4c8d11436a0049f6d2eee5b3b48ec27d6ce1b4ab8e71705c874fe6d384
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T01:06:38Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test leaks -- --exact a_hyphen_glued_token_is_refused_at_the_write_and_found_by_the_doctor for TC-records-a-hyphen-glued-token-is-a-leak: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4a295f06742c, observed 2026-11-18T01:06:38Z. The raw log stays in the machine-local store.
