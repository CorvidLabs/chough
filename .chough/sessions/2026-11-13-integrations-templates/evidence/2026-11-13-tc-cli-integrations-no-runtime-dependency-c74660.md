---
kind: evidence
id: 2026-11-13-tc-cli-integrations-no-runtime-dependency-c74660
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
revision: 5219fe8daacaa2baeb870782955b1f06090c072f
content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
environment: local-macos-arm64
fingerprint: sha256:7c5c8b8e6b9cf93d63a5d4ae9d2d5996fd515d3f6e4a16f6ebf7d0d0a844d39a
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T14:50:05Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5219fe8daaca, observed 2026-11-13T14:50:05Z. The raw log stays in the machine-local store.
