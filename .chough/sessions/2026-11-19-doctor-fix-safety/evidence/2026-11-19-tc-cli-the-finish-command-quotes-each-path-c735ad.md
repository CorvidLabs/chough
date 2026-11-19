---
kind: evidence
id: 2026-11-19-tc-cli-the-finish-command-quotes-each-path-c735ad
aliases: []
claim: "Ran cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out for TC-cli-the-finish-command-quotes-each-path: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-the-finish-command-quotes-each-path]
case_digest: [d68ddc8fbac742b62d0ddb225cac20b80eb7dc0468253d538d1ea05adb2ffaf5]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - the_finish_command_quotes_each_path_and_leaves_metadata_out
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:2c014c080160bef69639e932d5241aa2c952df75cbda4938f0b33294a341ba4c
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:43:27Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out for TC-cli-the-finish-command-quotes-each-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:43:27Z. The raw log stays in the machine-local store.
