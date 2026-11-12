---
kind: evidence
id: 2026-11-12-tc-cli-init-unbounded-knowledge-refused
aliases: []
claim: "Ran cargo test --locked --test knowledge_default -- --exact init_still_succeeds_and_writes_no_knowledge_when_the_index_cannot_be_bounded for TC-cli-init-unbounded-knowledge-refused: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-init-unbounded-knowledge-refused]
case_digest: [73c897b80fccae91f954f3d474455d744deaa08ae25944d9e99fb6a6e9a95390]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - knowledge_default
  - "--"
  - "--exact"
  - init_still_succeeds_and_writes_no_knowledge_when_the_index_cannot_be_bounded
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e871d36fc1a39009bee4ea40456fe2ada51a00d2
content: fb8e7010230dde4e3a9852ba896a428fbdfe2ff283e76a77d8072062b094ee44
environment: local-macos-arm64
fingerprint: sha256:418b9f9012821e635ab7f3849663236462af27a7401c70cafa2363918fdee82a
criteria: [KNOW-1, KNOW-4]
requirements: [REQ-cli-043]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T14:59:51Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test knowledge_default -- --exact init_still_succeeds_and_writes_no_knowledge_when_the_index_cannot_be_bounded for TC-cli-init-unbounded-knowledge-refused: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e871d36fc1a3, observed 2026-11-12T14:59:51Z. The raw log stays in the machine-local store.
