---
kind: evidence
id: 2026-11-15-tc-cli-build-reference-from-main-known-outdated
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact build_reference_from_main_4daf861_is_known_outdated for TC-cli-build-reference-from-main-known-outdated: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-build-reference-from-main-known-outdated]
case_digest: [e87e897bdf1ab06ffab9894326cd28ecb9b9faed7c463e3dc6c369087343f52f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - build_reference_from_main_4daf861_is_known_outdated
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:08d19a50f94085945fc0c4c2a098355786c90d65a3b1dc9b0d8ee3cd96edda44
criteria: [LIFE-11, LIFE-4]
requirements: [REQ-cli-034]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:11Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test cli -- --exact build_reference_from_main_4daf861_is_known_outdated for TC-cli-build-reference-from-main-known-outdated: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:11Z. The raw log stays in the machine-local store.
