---
kind: evidence
id: 2026-11-13-tc-cli-integrations-skill-file-exists-after-init-bda2c8
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init for TC-cli-integrations-skill-file-exists-after-init: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-skill-file-exists-after-init]
case_digest: [23f650f2098c747e712edc8684d3bda735c787475ceb29c5d44401c9fe1ad543]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - each_templates_skill_file_exists_after_its_own_chough_init
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: c0e3c340ae4fce048f37442831d242d42eeeef28
content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
environment: local-macos-arm64
fingerprint: sha256:8d8d4ff1ad0a9bb9e160071315434bb26d5e4e5a24a1375fc87f4b64ce2800c7
criteria: [LIFE-22, LIFE-24]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:19:47Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init for TC-cli-integrations-skill-file-exists-after-init: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c0e3c340ae4f, observed 2026-11-13T06:19:47Z. The raw log stays in the machine-local store.
