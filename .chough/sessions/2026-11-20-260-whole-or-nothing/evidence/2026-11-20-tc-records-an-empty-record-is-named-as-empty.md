---
kind: evidence
id: 2026-11-20-tc-records-an-empty-record-is-named-as-empty
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
revision: fcba5f1c433bff099b257add834061ef20d128d1
content: 8b4498351d7422a99f5e3262da5e5b6b8289dda8cfd3d68393d7cf783bdbfaa1
environment: local-macos-arm64
fingerprint: sha256:fde8ca5b0d33791db406daa875f0e6b51c06eec2d02712ed2c9e8fbd42d0d491
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:58:28Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty for TC-records-an-empty-record-is-named-as-empty: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fcba5f1c433b, observed 2026-11-20T16:58:28Z. The raw log stays in the machine-local store.
