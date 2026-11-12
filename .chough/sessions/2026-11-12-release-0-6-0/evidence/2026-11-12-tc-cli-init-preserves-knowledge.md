---
kind: evidence
id: 2026-11-12-tc-cli-init-preserves-knowledge
aliases: []
claim: "Ran cargo test --locked --test knowledge_default -- --exact init_preserves_an_existing_knowledge_directory for TC-cli-init-preserves-knowledge: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-init-preserves-knowledge]
case_digest: [7029aede2023d976e97e9ef147f8b4c3b59ad8c7c4d49598c70376562a54436f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - knowledge_default
  - "--"
  - "--exact"
  - init_preserves_an_existing_knowledge_directory
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: e871d36fc1a39009bee4ea40456fe2ada51a00d2
content: fb8e7010230dde4e3a9852ba896a428fbdfe2ff283e76a77d8072062b094ee44
environment: local-macos-arm64
fingerprint: sha256:69058f5841a6c66dc6672e9f7d1e0da9f02577c1f371df06e2521f9f1bdef236
criteria: [KNOW-1, KNOW-4]
requirements: [REQ-cli-043]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T14:59:48Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test knowledge_default -- --exact init_preserves_an_existing_knowledge_directory for TC-cli-init-preserves-knowledge: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e871d36fc1a3, observed 2026-11-12T14:59:48Z. The raw log stays in the machine-local store.
