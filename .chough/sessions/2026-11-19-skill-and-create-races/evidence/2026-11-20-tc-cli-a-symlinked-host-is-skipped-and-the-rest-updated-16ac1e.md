---
kind: evidence
id: 2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated-16ac1e
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
revision: f38abb556cee50e587a4c2446a91c7f2fa140997
content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
environment: local-macos-arm64
fingerprint: sha256:58331d588ac8a9111bff475363ca19a1abda4af616f49a71360d54b0603e1643
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:59:49Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated for TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f38abb556cee, observed 2026-11-20T14:59:49Z. The raw log stays in the machine-local store.
