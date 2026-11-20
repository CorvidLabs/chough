---
kind: evidence
id: 2026-11-20-tc-records-an-empty-record-is-named-as-empty-113250
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
revision: b5e461265cb8c859ece2aefd1f75db8441b213cd
content: 47f48f674af2b26873198e18d643cde1bc9388765919f10320e0d91364a769a5
environment: local-macos-arm64
fingerprint: sha256:dc7e2ea6aaddb38c01baef7efa5f8a7eaf67373bf8425d90dd3aae560ae1da67
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T18:49:16Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test records -- --exact an_empty_record_is_named_as_empty for TC-records-an-empty-record-is-named-as-empty: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b5e461265cb8, observed 2026-11-20T18:49:16Z. The raw log stays in the machine-local store.
