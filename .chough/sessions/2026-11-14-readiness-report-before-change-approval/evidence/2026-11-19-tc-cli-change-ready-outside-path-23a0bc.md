---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-outside-path-23a0bc
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
revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
environment: local-macos-arm64
fingerprint: sha256:f03a3082c6fe15f2327e0b9cc5cffcde1a5b308921157edf2b1b6574ae8e45ff
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:14:51Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_changed_file_outside_the_declared_paths_is_named for TC-cli-change-ready-outside-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5b9c1b4ad6f2, observed 2026-11-19T15:14:51Z. The raw log stays in the machine-local store.
