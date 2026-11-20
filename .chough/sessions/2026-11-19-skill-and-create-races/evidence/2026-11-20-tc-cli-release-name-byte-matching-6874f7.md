---
kind: evidence
id: 2026-11-20-tc-cli-release-name-byte-matching-6874f7
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
revision: f38abb556cee50e587a4c2446a91c7f2fa140997
content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
environment: local-macos-arm64
fingerprint: sha256:091f020a3dca7e2c91f3c3324b25cdca30987e29611fe206ead07fa2c1a86b79
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T15:00:16Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test skill_releases -- --exact recognition_requires_the_filename_and_exact_unnormalized_bytes for TC-cli-release-name-byte-matching: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f38abb556cee, observed 2026-11-20T15:00:16Z. The raw log stays in the machine-local store.
