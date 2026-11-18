---
kind: evidence
id: 2026-11-18-tc-history-a-receipt-keeps-its-entrys-criteria-requirements-21b18c
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
duration_seconds: 4
revision: 730a5e34614756d1a5b194373012ccf3bd4c7192
content: 352cc3205d90bdc57167a5e60738f4e7935812785574d1acd47a55396a359f1a
environment: local-macos-arm64
fingerprint: sha256:5b10fabe36fe2aeb98fc16d96e0144ca1053cbe46849cf3b142f46cee29e272a
criteria: [KNOW-35]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:42:52Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-receipt-keeps-its-entrys-criteria-requirements-and-limitations: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 730a5e346147, observed 2026-11-18T20:42:52Z. The raw log stays in the machine-local store.
