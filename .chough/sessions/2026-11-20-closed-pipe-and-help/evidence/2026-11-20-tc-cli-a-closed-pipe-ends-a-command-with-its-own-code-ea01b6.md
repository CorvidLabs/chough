---
kind: evidence
id: 2026-11-20-tc-cli-a-closed-pipe-ends-a-command-with-its-own-code-ea01b6
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact a_command_whose_reader_has_gone_exits_with_its_own_code for TC-cli-a-closed-pipe-ends-a-command-with-its-own-code: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-closed-pipe-ends-a-command-with-its-own-code]
case_digest: [97eec6154678858041dbbc6d362d2403a6e9502b333aee306f2e5701b834ea72]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - a_command_whose_reader_has_gone_exits_with_its_own_code
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: e60f19787d96c9a0ebcf9399d93a04939d4696fb
content: 1640f508756b07e5571731954b75bcad62ee1ad2bddb05f87a5dd462354e2c55
environment: local-macos-arm64
fingerprint: sha256:ad6164a4d779652e4046ad4514454ce6104521746cc769a7d54b0e810b6db7d7
criteria: [LIFE-4]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:07:13Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_command_whose_reader_has_gone_exits_with_its_own_code for TC-cli-a-closed-pipe-ends-a-command-with-its-own-code: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e60f19787d96, observed 2026-11-20T17:07:13Z. The raw log stays in the machine-local store.
