---
kind: evidence
id: 2026-11-18-tc-cli-change-ready-outside-path
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-outside-path]
case_digest: [08fb3fc5bc3f9e56789e531095851ca120f520c96e45325f5fa7d31eaf028f6f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_changed_file_outside_the_declared_paths_is_named
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: c817d3d44418dd62d69d63ec8e3d48c3b5b4d709
content: 6cabd8c66b38e712b6a07a902e5b8a0f74291b6ff8ed75aee25f4a80dea63f82
environment: local-macos-arm64
fingerprint: sha256:cf0aee7a879d09776baeb67e618113ac01602a27d2c576dc1df98ee279433a63
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T14:59:07Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c817d3d44418, observed 2026-11-18T14:59:07Z. The raw log stays in the machine-local store.
