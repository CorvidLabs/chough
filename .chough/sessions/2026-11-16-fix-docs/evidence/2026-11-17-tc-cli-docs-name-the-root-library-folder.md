---
kind: evidence
id: 2026-11-17-tc-cli-docs-name-the-root-library-folder
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact the_knowledge_table_names_the_repository_root_library for TC-cli-docs-name-the-root-library-folder: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-docs-name-the-root-library-folder]
case_digest: [18c1c042ce77a0f6567e18b7d574016a76e1d6d7f05572ec5ba4cd30d6b1a50a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - the_knowledge_table_names_the_repository_root_library
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:34182e8df5c17da1e2132b09eb435de1d888f9c057ed0c2014e39f43d1f734b1
criteria: [KNOW-1, KNOW-5]
requirements: [REQ-cli-043]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:40Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test docs -- --exact the_knowledge_table_names_the_repository_root_library for TC-cli-docs-name-the-root-library-folder: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:40Z. The raw log stays in the machine-local store.
