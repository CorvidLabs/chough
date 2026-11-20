---
kind: evidence
id: 2026-11-20-tc-records-migrate-names-every-returning-file-in-one-refusal-4bcd95
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact returning_files_with_other_bytes_are_all_named_in_one_refusal for TC-records-migrate-names-every-returning-file-in-one-refusal: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-names-every-returning-file-in-one-refusal]
case_digest: [608c0d0ceb5ba1f61b81faeea361b40abbe25d69d36139596a755d00c15ea572]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - returning_files_with_other_bytes_are_all_named_in_one_refusal
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:1cfaaa361bb3e98dd26bbc4052fac79520eb0f6a14f16851f11750ab08ee70ce
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:58:35Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact returning_files_with_other_bytes_are_all_named_in_one_refusal for TC-records-migrate-names-every-returning-file-in-one-refusal: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T03:58:35Z. The raw log stays in the machine-local store.
