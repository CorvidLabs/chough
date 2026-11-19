---
kind: evidence
id: 2026-11-19-tc-cli-the-finish-command-quotes-each-path-0ab8d3
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
duration_seconds: 5
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:a1916486763a3f36e61913eb54245224d14f85efd29bf6d5407ad49e25b78a4f
criteria: [DRIVE-17]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:12:51Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out for TC-cli-the-finish-command-quotes-each-path: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:12:51Z. The raw log stays in the machine-local store.
