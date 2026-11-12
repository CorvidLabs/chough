---
kind: evidence
id: 2026-11-12-tc-cli-init-claude-host-prints-the-claude-md-line-e06091
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
revision: 70468c0a5ae1d54d71da2bb408d72b4af32c7447
content: b084fb6e7052b84f55c61038681d6e36b4e77bcce77324002537f5d357214a84
environment: local-macos-arm64
fingerprint: sha256:694e4cb1ce2e376a7811ae5f212c66fdba0a99940238a60e150d1cc11ee00901
criteria: [LIFE-12, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:15:56Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test optional -- --exact init_with_the_claude_host_prints_the_claude_md_line_and_writes_none for TC-cli-init-claude-host-prints-the-claude-md-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 70468c0a5ae1, observed 2026-11-12T22:15:56Z. The raw log stays in the machine-local store.
