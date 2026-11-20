---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files-e4c078
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-on-the-default-branch-names-own-files]
case_digest: [0cb9bc7a1551cc2ca552005b982baf6dcb01712bb51e8fbfd1c12d8a3b0a2f12]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - on_the_default_branch_itself_the_change_s_files_are_still_named
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 9c79d7694223aa3acc2027e8f95d03edb888ab16
content: a5023893b74f5a383e037635a88a59ffabfdf2c3a6e06852324b3460eec94e03
environment: local-macos-arm64
fingerprint: sha256:35541848f4be7ec19356ef1f040cc2fbf636a78ec9cc17624ba06bf667359a10
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:41:41Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9c79d7694223, observed 2026-11-20T06:41:41Z. The raw log stays in the machine-local store.
