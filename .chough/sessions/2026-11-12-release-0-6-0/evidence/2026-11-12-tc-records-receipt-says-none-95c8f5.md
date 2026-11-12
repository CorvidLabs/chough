---
kind: evidence
id: 2026-11-12-tc-records-receipt-says-none-95c8f5
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_receipt_section_with_nothing_to_list_says_none for TC-records-receipt-says-none: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-receipt-says-none]
case_digest: [c4afb92774b187f974b79222fd3f91cefc9c62739befa6382166318807e3a5af]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_receipt_section_with_nothing_to_list_says_none
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 06e0a1b0335e7c75b623567e77a8a9d38e087433
content: 10367cd0474ee6f739d7a93127ee5726844cc001fb604227fc2f374d06f2ca9c
environment: local-macos-arm64
fingerprint: sha256:a1c20198a809028ae75ecfdcad97bc2ee9719ef3967b996f5035a273dd0cdd58
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:47:40Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact a_receipt_section_with_nothing_to_list_says_none for TC-records-receipt-says-none: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 06e0a1b0335e, observed 2026-11-12T17:47:40Z. The raw log stays in the machine-local store.
