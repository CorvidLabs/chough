---
kind: evidence
id: 2026-11-17-tc-cli-change-ship-cut-summary-names-the-cut-and-json
aliases: []
claim: "Ran cargo test --locked --bin chough -- --exact change::tests::the_summary_names_the_cut_and_json_when_it_is_longer_than_twelve_lines for TC-cli-change-ship-cut-summary-names-the-cut-and-json: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ship-cut-summary-names-the-cut-and-json]
case_digest: [c0bc08a29946966973bf420c153b3e39030e1df5e70f3728c9bb8138ef3fd820]
command:
  - cargo
  - test
  - "--locked"
  - "--bin"
  - chough
  - "--"
  - "--exact"
  - change::tests::the_summary_names_the_cut_and_json_when_it_is_longer_than_twelve_lines
exit_code: 0
duration_seconds: 4
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:f8ed5513533503df53d9d59628b94d7b2148022b4ff0c0f20015849561091aef
criteria: [DRIVE-20]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-17T14:22:23Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --bin chough -- --exact change::tests::the_summary_names_the_cut_and_json_when_it_is_longer_than_twelve_lines for TC-cli-change-ship-cut-summary-names-the-cut-and-json: passed (exit 0).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:23Z. The raw log stays in the machine-local store.
