---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-outside-path
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
revision: 24323c5c04434521dfff65da788125d02010f545
content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
environment: local-macos-arm64
fingerprint: sha256:0dd2f536313e34d88f8e418b032060d1150336cffdb869f83060dea1d7bc7fcc
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:17:04Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 24323c5c0443, observed 2026-11-19T02:17:04Z. The raw log stays in the machine-local store.
