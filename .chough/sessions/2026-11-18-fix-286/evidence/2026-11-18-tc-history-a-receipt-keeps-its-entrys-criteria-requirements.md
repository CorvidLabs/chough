---
kind: evidence
id: 2026-11-18-tc-history-a-receipt-keeps-its-entrys-criteria-requirements
aliases: []
claim: "Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-receipt-keeps-its-entrys-criteria-requirements-and-limitations: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-receipt-keeps-its-entrys-criteria-requirements-and-limitations]
case_digest: [9c52bbbed227ddcaa2a9f1077890db6f3fc979cbee7055604a505a00b3d33bad]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - evidence
  - "--"
  - "--exact"
  - a_story_receipt_carries_each_field_its_entry_knows
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 75bc2f3c1e8b3ccbc8bc3f05cdfcb1219ff8a112
content: 052972a2b803cc984adb57218fb0eebd850f621c367ea6ff1b3c637427b876da
environment: local-macos-arm64
fingerprint: sha256:52ea2cf8e9dbd3f157149958f2fbec19be20cd8c5e95affad56cdcd5d337866e
criteria: [KNOW-35]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:57:44Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-receipt-keeps-its-entrys-criteria-requirements-and-limitations: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 75bc2f3c1e8b, observed 2026-11-18T19:57:44Z. The raw log stays in the machine-local store.
