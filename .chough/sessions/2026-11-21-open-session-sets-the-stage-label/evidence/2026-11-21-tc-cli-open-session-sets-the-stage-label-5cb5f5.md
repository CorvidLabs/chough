---
kind: evidence
id: 2026-11-21-tc-cli-open-session-sets-the-stage-label-5cb5f5
aliases: []
claim: "Ran cargo test --locked --test github_labels -- --exact the_script_follows_an_open_session_ahead_of_an_archived_one for TC-cli-open-session-sets-the-stage-label: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-open-session-sets-the-stage-label]
case_digest: [ff3a404cb5b38745504b41f403911d9aa4f5e422cd5795552a4b4148841f11fb]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - github_labels
  - "--"
  - "--exact"
  - the_script_follows_an_open_session_ahead_of_an_archived_one
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 6bc60913b028761b20832a6906dc8e31d17e70df
content: 540e9aa15ae2641e45f5b3e026b5135a4c7132e33a6c799de14ba1a5ee8d8f21
environment: local-macos-arm64
fingerprint: sha256:95310f0bfe8aaf867e3933b4951adb106e216b8eb82e3b60ed66924f88a7353b
criteria: [LIFE-32, LIFE-31]
requirements: [REQ-cli-070]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-21T18:15:02Z
actor:
  - terminal: t10j
---
**Ran cargo test --locked --test github_labels -- --exact the_script_follows_an_open_session_ahead_of_an_archived_one for TC-cli-open-session-sets-the-stage-label: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6bc60913b028, observed 2026-11-21T18:15:02Z. The raw log stays in the machine-local store.
