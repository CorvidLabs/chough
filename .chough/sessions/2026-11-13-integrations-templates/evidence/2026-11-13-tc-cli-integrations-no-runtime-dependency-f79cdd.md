---
kind: evidence
id: 2026-11-13-tc-cli-integrations-no-runtime-dependency-f79cdd
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
revision: c0e3c340ae4fce048f37442831d242d42eeeef28
content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
environment: local-macos-arm64
fingerprint: sha256:994a2da66035c1731debe782f93f2aacc823dc9cbb3ea3d02269aae63c0e5ae2
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:19:38Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c0e3c340ae4f, observed 2026-11-13T06:19:38Z. The raw log stays in the machine-local store.
