---
kind: evidence
id: 2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated-f546f9
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
revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
environment: local-macos-arm64
fingerprint: sha256:deefcac33bdb9c7d1814f287df7a51fcb8e67716e8f05e089c8369086b250146
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:35:44Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated for TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cece9573c0c, observed 2026-11-20T14:35:44Z. The raw log stays in the machine-local store.
