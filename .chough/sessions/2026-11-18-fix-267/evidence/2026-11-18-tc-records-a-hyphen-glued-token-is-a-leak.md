---
kind: evidence
id: 2026-11-18-tc-records-a-hyphen-glued-token-is-a-leak
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
duration_seconds: 1
revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
content: c91c6308e94322af268ba5d50c9fe2e1d8eb431d48b042b1dd9621dc9f501d46
environment: local-macos-arm64
fingerprint: sha256:86f2c9d271269339f4cbc2edd85aa69d9ebf9027ccb42547696ab22b9fa12c3c
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T00:36:59Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test leaks -- --exact a_hyphen_glued_token_is_refused_at_the_write_and_found_by_the_doctor for TC-records-a-hyphen-glued-token-is-a-leak: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8abb5bb89b52, observed 2026-11-18T00:36:59Z. The raw log stays in the machine-local store.
