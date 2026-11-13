---
kind: evidence
id: 2026-11-13-tc-cli-integrations-skill-file-exists-after-init-fd8cfb
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
duration_seconds: 1
revision: 5219fe8daacaa2baeb870782955b1f06090c072f
content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
environment: local-macos-arm64
fingerprint: sha256:45d91e55a199c2a89a8f09c86ab203129022f0de9e4a888958d05c8ff098ec2c
criteria: [LIFE-22, LIFE-24]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T14:50:11Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init for TC-cli-integrations-skill-file-exists-after-init: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5219fe8daaca, observed 2026-11-13T14:50:11Z. The raw log stays in the machine-local store.
