---
kind: evidence
id: 2026-11-19-tc-cli-the-finish-command-quotes-each-path-57dcf0
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
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:1fd90fb1398e9a5106a2cbf2a34a2e94be4d9e6830380d2cd05944f96161e429
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:15:12Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out for TC-cli-the-finish-command-quotes-each-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:15:12Z. The raw log stays in the machine-local store.
