---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-outside-path-36e6bd
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
duration_seconds: 1
revision: 240d7dafabeea57a669447149f515f63f6d3e15a
content: fb0d299c3c4131c6b665208c44fbf8a08a201489a1a673283400632ef7aad137
environment: local-macos-arm64
fingerprint: sha256:53538f3bfb20ebbb0bf025d749b0259460c1d0b7b8bf66293a1a9ba04b3ce103
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:13:46Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 240d7dafabee, observed 2026-11-19T03:13:46Z. The raw log stays in the machine-local store.
