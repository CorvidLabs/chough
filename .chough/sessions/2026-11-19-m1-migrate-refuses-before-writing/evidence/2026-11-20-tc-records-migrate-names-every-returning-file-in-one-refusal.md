---
kind: evidence
id: 2026-11-20-tc-records-migrate-names-every-returning-file-in-one-refusal
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
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:ac81b237743fac27d59093dc2670315707e2ffb0549994326a2e2186c1d4ba17
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:23:40Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact returning_files_with_other_bytes_are_all_named_in_one_refusal for TC-records-migrate-names-every-returning-file-in-one-refusal: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:23:40Z. The raw log stays in the machine-local store.
