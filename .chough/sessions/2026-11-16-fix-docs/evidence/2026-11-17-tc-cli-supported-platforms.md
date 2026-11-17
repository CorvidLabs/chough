---
kind: evidence
id: 2026-11-17-tc-cli-supported-platforms
aliases: []
claim: "Ran cargo test --locked --test package -- --exact supported_platforms_are_macos_and_linux_with_windows_through_wsl for TC-cli-supported-platforms: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-supported-platforms]
case_digest: [112ae5cf4fc7779115cab412f367a1e214a5e2e583290f4df406392f277341e5]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - package
  - "--"
  - "--exact"
  - supported_platforms_are_macos_and_linux_with_windows_through_wsl
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:8eb735173ce495247a6c47b995d71d2283354982aa7f828e094175939787d9f8
criteria: [FLOW-17, LIFE-9]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:40Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test package -- --exact supported_platforms_are_macos_and_linux_with_windows_through_wsl for TC-cli-supported-platforms: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:40Z. The raw log stays in the machine-local store.
