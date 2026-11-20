---
kind: evidence
id: 2026-11-20-tc-cli-a-closed-pipe-ends-a-command-with-its-own-code
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
duration_seconds: 2
revision: be53e668fdf94ba61ff800b3702fc70d89517b82
content: 34322c35469b860b51450cad40a3b0e14c5024c1f945e7dbebfcf2ce4563c892
environment: local-macos-arm64
fingerprint: sha256:5610ae910084beebb711acfb46e45017a256679ee25be5068cd423e780fa2f24
criteria: [LIFE-4]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:44:25Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_command_whose_reader_has_gone_exits_with_its_own_code for TC-cli-a-closed-pipe-ends-a-command-with-its-own-code: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision be53e668fdf9, observed 2026-11-20T16:44:25Z. The raw log stays in the machine-local store.
