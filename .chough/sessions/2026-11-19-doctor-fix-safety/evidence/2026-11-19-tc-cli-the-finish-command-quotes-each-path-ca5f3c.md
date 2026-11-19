---
kind: evidence
id: 2026-11-19-tc-cli-the-finish-command-quotes-each-path-ca5f3c
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
duration_seconds: 1
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:e19b922c2ed45e7de21e74d8989af8186a4143a92a33cafed7741d190363c0ec
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:50:14Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out for TC-cli-the-finish-command-quotes-each-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:50:14Z. The raw log stays in the machine-local store.
