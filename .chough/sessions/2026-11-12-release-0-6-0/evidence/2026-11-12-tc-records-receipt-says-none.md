---
kind: evidence
id: 2026-11-12-tc-records-receipt-says-none
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_receipt_section_with_nothing_to_list_says_none for TC-records-receipt-says-none: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: caa29f5e704c512b9af03b0ea33bdaaff1fc962c
content: 9db289c4ce1f8d5f1ad79c80bb5b911b1f95526ca85ae2f905c26e24c0c96e59
environment: local-macos-arm64
fingerprint: sha256:7ae2d473719f722d83030dad90b945d4630f2698a029e33a6baed82df6b40863
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:45:43Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact a_receipt_section_with_nothing_to_list_says_none for TC-records-receipt-says-none: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision caa29f5e704c, observed 2026-11-12T17:45:43Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_receipt_section_with_nothing_to_list_says_none stdout ----
- thread 'a_receipt_section_with_nothing_to_list_says_none' (921116580) panicked at tests/migrate.rs:1451:9:
- assertion `left == right` failed: ## Redactions in
- ---
- kind: receipt
- id: 2026-11-12-from-0-5
- plan_digest: e6a29c8e99779a298fa0f120b45cd53c7f2c4051fb6e743f867302a9a6adccbe
- applied: 2026-11-12T17:45:41Z
- consent: "--consent (a relayed claim): SYNTHETIC-human said migrate it (fixture)"
- removed_at_commit: 32cfd9e862846aa890bfa71254845f4b30c50265
- ---
- # Migration from Chough 0.5.x
- Git history still holds what earlier commits recorded; this migration does not rewrite it.
- ## Before
- - `.chough/history/cycles/CYC-59cf8cb7d24ff7e8532b03ab/EVT-577d9677fecbb00805de3767.json` ebbcc6ea7f774d96a2813dab2d8994c6c3855c39dce8ca73af822c72108c09b0
- - `.chough/history/cycles/CYC-59cf8cb7d24ff7e8532b03ab/EVT-b2021d7321b225e4715ffbc3.json` c13542877bfd615c88c4d850920fe239eebcb2704f0d01bb43578efa97f0e858
- - `.chough/history/cycles/CYC-59cf8cb7d24ff7e8532b03ab/EVT-c646b328897a087e36e3f568.json` 29aa89fdcb871f9be86fa5f8f0c3af9062977547907ba72ecae4e0a0bdc214fa
- - `.chough/sessions/.gitignore` 33887a1ea91c394fc4c263371641dba44db652b9defb90020f46783706a04024
- - `.chough/sessions/archive/fixture-second.json` 06a173b60a7d9121f542478c5b9f900e06f8511bd906bb6bcd2aae7a8ee7e92b
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 21 filtered out; finished in 0.85s
