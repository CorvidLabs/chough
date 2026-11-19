---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-outside-path-801e59
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
revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
environment: local-macos-arm64
fingerprint: sha256:0ee95669148ce9c534a573da6d46c4322d54dd3073b008779c77ff7e74fbba0a
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:38:27Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e2c643fed4b, observed 2026-11-19T04:38:27Z. The raw log stays in the machine-local store.
