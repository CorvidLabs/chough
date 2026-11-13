---
kind: evidence
id: 2026-11-13-tc-cli-integrations-no-runtime-dependency-819e19
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
revision: 12f2cafe777a48ef6e6cb92c9435997ab5b4e0d2
content: dd44a2f14e6358d057ef934649d20ff96745157b0280619cc43ef2f0e973b2c4
environment: local-macos-arm64
fingerprint: sha256:aedf96ca8855423ef03e533464becdbe919981be1a84584cfaaef9c3c8d99e0b
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:14:12Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 12f2cafe777a, observed 2026-11-13T04:14:12Z. The raw log stays in the machine-local store.
