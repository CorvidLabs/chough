---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-outside-path-ef4054
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
duration_seconds: 2
revision: e42c96fec77c56feff65256be9e1d5deb1f34742
content: f0d0677ea6983f3c4141ac589c1e4ec1b34b9f51b207be3056be35da0140df8c
environment: local-macos-arm64
fingerprint: sha256:6492c4d9a3bfbcb8b0a80b7ac3cab82908b9906bc3ebdfca44fe68985b17b29a
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:12:39Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e42c96fec77c, observed 2026-11-19T04:12:39Z. The raw log stays in the machine-local store.
