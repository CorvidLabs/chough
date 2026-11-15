---
kind: evidence
id: 2026-11-15-tc-cli-knowledge-rebuild-keeps-observation-current
aliases: []
claim: "Ran cargo test --locked --test evidence -- --exact rebuilding_the_knowledge_index_keeps_the_observation_current for TC-cli-knowledge-rebuild-keeps-observation-current: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-knowledge-rebuild-keeps-observation-current]
case_digest: [1a8cf4ac50403134723c7756b10d141bf8cbbac4d4351a16db275eb5f6ff9706]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - evidence
  - "--"
  - "--exact"
  - rebuilding_the_knowledge_index_keeps_the_observation_current
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:55540156848438f448de1a9dd959b5addfbcdc165acab10f11b9edd6ff38d188
criteria: [LIFE-7]
requirements: [REQ-cli-004]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:08Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test evidence -- --exact rebuilding_the_knowledge_index_keeps_the_observation_current for TC-cli-knowledge-rebuild-keeps-observation-current: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:08Z. The raw log stays in the machine-local store.
