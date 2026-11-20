---
kind: evidence
id: 2026-11-20-tc-cli-a-write-into-a-closed-pipe-saves-and-exits-0
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
duration_seconds: 0
revision: be53e668fdf94ba61ff800b3702fc70d89517b82
content: 34322c35469b860b51450cad40a3b0e14c5024c1f945e7dbebfcf2ce4563c892
environment: local-macos-arm64
fingerprint: sha256:697cc383eb45bda27d7d22eddc376b9e79223f201b5b3831b16e62a4f8254f35
criteria: [LIFE-4]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:44:27Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_write_into_a_closed_pipe_saves_and_exits_0 for TC-cli-a-write-into-a-closed-pipe-saves-and-exits-0: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision be53e668fdf9, observed 2026-11-20T16:44:27Z. The raw log stays in the machine-local store.
