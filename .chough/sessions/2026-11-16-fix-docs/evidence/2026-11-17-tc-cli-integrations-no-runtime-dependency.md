---
kind: evidence
id: 2026-11-17-tc-cli-integrations-no-runtime-dependency
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-no-runtime-dependency]
case_digest: [f7f634f29e7ef6671beb8a7fec69289f22983cf4ba5d71e54bf2be64e7d9daf5]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:019c6cb7d722352e6be5a82b8cec56fa45791c9d43ede7177674534bbbbf76cc
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:35Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:35Z. The raw log stays in the machine-local store.
