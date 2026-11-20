---
kind: evidence
id: 2026-11-20-tc-records-migrate-names-every-returning-file-in-one-refusal-f6d6c2
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
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:bba8b912c3b6235ce7ced3581d20f13cac0f9f0e40efa8561268e52478a374da
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:35:44Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact returning_files_with_other_bytes_are_all_named_in_one_refusal for TC-records-migrate-names-every-returning-file-in-one-refusal: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:35:44Z. The raw log stays in the machine-local store.
