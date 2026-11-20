---
kind: evidence
id: 2026-11-20-tc-cli-release-name-byte-matching
aliases: []
claim: "Ran cargo test --locked --test skill_releases -- --exact recognition_requires_the_filename_and_exact_unnormalized_bytes for TC-cli-release-name-byte-matching: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-release-name-byte-matching]
case_digest: [a5acedd9a4928ad9ee4b772ed63a878118e614703a574b6996c4d19960f89387]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - skill_releases
  - "--"
  - "--exact"
  - recognition_requires_the_filename_and_exact_unnormalized_bytes
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
environment: local-macos-arm64
fingerprint: sha256:24811f306aeaa7cd1a94e2bd3c4b91204653feb87e2e50611ad4e0cc36f58aef
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:36:07Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test skill_releases -- --exact recognition_requires_the_filename_and_exact_unnormalized_bytes for TC-cli-release-name-byte-matching: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cece9573c0c, observed 2026-11-20T14:36:07Z. The raw log stays in the machine-local store.
