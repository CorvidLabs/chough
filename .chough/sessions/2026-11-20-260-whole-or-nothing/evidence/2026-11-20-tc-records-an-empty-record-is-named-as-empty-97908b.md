---
kind: evidence
id: 2026-11-20-tc-records-an-empty-record-is-named-as-empty-97908b
aliases: []
claim: "Ran cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty for TC-records-an-empty-record-is-named-as-empty: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-an-empty-record-is-named-as-empty]
case_digest: [b43130f1d46b0f8006089aed181ac9c3bd56c7612983b2c90e6061be05fc61d3]
command: [cargo, test, --locked, --test, records, --, --exact, an_empty_record_is_named_as_empty]
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 8418dff2f5fcfaff322fcee860159ad82e6a593d
content: 5ec53b8bda35b385cc4e52ed962166b271b604f4149eee7e0e597aa2f51a987c
environment: local-macos-arm64
fingerprint: sha256:d447e14763614a2a034a87b6e117ccf7f0482f69c8f67e7b207dbef993f28e57
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:56:46Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty for TC-records-an-empty-record-is-named-as-empty: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8418dff2f5fc, observed 2026-11-20T17:56:46Z. The raw log stays in the machine-local store.
