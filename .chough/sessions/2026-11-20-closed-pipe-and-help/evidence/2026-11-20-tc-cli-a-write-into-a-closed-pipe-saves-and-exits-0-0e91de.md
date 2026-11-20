---
kind: evidence
id: 2026-11-20-tc-cli-a-write-into-a-closed-pipe-saves-and-exits-0-0e91de
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact a_write_into_a_closed_pipe_saves_and_exits_0 for TC-cli-a-write-into-a-closed-pipe-saves-and-exits-0: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-write-into-a-closed-pipe-saves-and-exits-0]
case_digest: [085398e479db6955c5f57bd956079d1be0c5d429e80887a382b063bf0ac953b9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - a_write_into_a_closed_pipe_saves_and_exits_0
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: e60f19787d96c9a0ebcf9399d93a04939d4696fb
content: 1640f508756b07e5571731954b75bcad62ee1ad2bddb05f87a5dd462354e2c55
environment: local-macos-arm64
fingerprint: sha256:c95ea1311fbfe7082f067d8c5713002edff49f34a5213f27f109c04aefa3ef3a
criteria: [LIFE-4]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:07:16Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_write_into_a_closed_pipe_saves_and_exits_0 for TC-cli-a-write-into-a-closed-pipe-saves-and-exits-0: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e60f19787d96, observed 2026-11-20T17:07:16Z. The raw log stays in the machine-local store.
