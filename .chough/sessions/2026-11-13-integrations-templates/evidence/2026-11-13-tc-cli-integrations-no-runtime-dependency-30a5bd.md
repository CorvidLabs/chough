---
kind: evidence
id: 2026-11-13-tc-cli-integrations-no-runtime-dependency-30a5bd
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
revision: a442a2acc4a23948c031c36675861ba57f04c980
content: 2fb922c6ab66ac421f2cbd88feaa0ec2fd87004f296b827945dfb087f7231437
environment: local-macos-arm64
fingerprint: sha256:066f0024547be19cac62553e56cfe92c70a6161aa7104ef41280610f22d2f4a8
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:54:41Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a442a2acc4a2, observed 2026-11-13T04:54:41Z. The raw log stays in the machine-local store.
