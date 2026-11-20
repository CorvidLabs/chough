---
kind: evidence
id: 2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated for TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated]
case_digest: [f5dafab564def17561e196a24027c02c604deab6484670dc79ef812d6efc49a5]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - a_symlinked_host_is_skipped_and_the_rest_updated
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 63057710d3223425b0db8d988717f2c657e5035b
content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
environment: local-macos-arm64
fingerprint: sha256:58331d588ac8a9111bff475363ca19a1abda4af616f49a71360d54b0603e1643
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T09:20:34Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated for TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 63057710d322, observed 2026-11-20T09:20:34Z. The raw log stays in the machine-local store.
