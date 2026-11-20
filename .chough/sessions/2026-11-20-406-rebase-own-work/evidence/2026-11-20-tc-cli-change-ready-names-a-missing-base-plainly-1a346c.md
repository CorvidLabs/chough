---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly-1a346c
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-names-a-missing-base-plainly]
case_digest: [d6851ef47f5c60f0fba435275e008a048d4eba48aa7aaef9def0095aff34494f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_base_commit_the_repository_lacks_is_named_plainly
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 9c79d7694223aa3acc2027e8f95d03edb888ab16
content: a5023893b74f5a383e037635a88a59ffabfdf2c3a6e06852324b3460eec94e03
environment: local-macos-arm64
fingerprint: sha256:91d16610945ef55d192ecadeb1f5c30b6bc623291ca9011fc9495b093f8e5a0d
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:41:43Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9c79d7694223, observed 2026-11-20T06:41:43Z. The raw log stays in the machine-local store.
