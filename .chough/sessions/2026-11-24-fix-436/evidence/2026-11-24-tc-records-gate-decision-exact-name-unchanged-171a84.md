---
kind: evidence
id: 2026-11-24-tc-records-gate-decision-exact-name-unchanged-171a84
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact an_exact_name_close_renders_the_display_name_unchanged for TC-records-gate-decision-exact-name-unchanged: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-gate-decision-exact-name-unchanged]
case_digest: [dcfbbd40feb12f2e8f542756d20174c7465e14c229bd282a7c5dda528702eeac]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - an_exact_name_close_renders_the_display_name_unchanged
exit_code: 0
duration_seconds: 0
revision: 16230c5626ce08ab2e9ffdf0447b9e40b67e3c17
content: 703787281d7e1c5367cb914878749d367144e87ca0b4b399bec417985d838868
environment: local-macos-arm64
fingerprint: sha256:c2be898a1c3cc1ace2521a6707b7ef806e168ab42cf2253a808d04763921ad4c
criteria: [FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-24T04:51:46Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test humans -- --exact an_exact_name_close_renders_the_display_name_unchanged for TC-records-gate-decision-exact-name-unchanged: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 16230c5626ce, observed 2026-11-24T04:51:46Z. The raw log stays in the machine-local store.
