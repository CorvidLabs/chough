---
kind: evidence
id: 2026-11-13-tc-cli-init-claude-host-prints-the-claude-md-line-425cda
aliases: []
claim: "Ran cargo test --locked --test optional -- --exact init_with_the_claude_host_prints_the_claude_md_line_and_writes_none for TC-cli-init-claude-host-prints-the-claude-md-line: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-init-claude-host-prints-the-claude-md-line]
case_digest: [3eac1ceb4d2119359f445c1f9d95150a4644fb4c90fca9be1a964dafedff4106]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - optional
  - "--"
  - "--exact"
  - init_with_the_claude_host_prints_the_claude_md_line_and_writes_none
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:8cf779ff1b1adba04bcd44fe73d815da8a4a6e5d2ad3e988b9e775b25ca098d5
criteria: [LIFE-12, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:48:40Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test optional -- --exact init_with_the_claude_host_prints_the_claude_md_line_and_writes_none for TC-cli-init-claude-host-prints-the-claude-md-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:48:40Z. The raw log stays in the machine-local store.
