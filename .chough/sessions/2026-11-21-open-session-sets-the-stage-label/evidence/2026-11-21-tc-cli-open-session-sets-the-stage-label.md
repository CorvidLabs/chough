---
kind: evidence
id: 2026-11-21-tc-cli-open-session-sets-the-stage-label
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
content: d30d4d763d1167a47980a64dd0a82060d63b7a49fa34428e37150132597e159f
environment: local-macos-arm64
fingerprint: sha256:1670c9bfaf7c9dfa18ecd23756028b0edcb743c39684203abe2663f09bc0c67b
criteria: [LIFE-32, LIFE-31]
requirements: [REQ-cli-070]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-21T18:14:10Z
actor:
  - terminal: t10j
---
**Ran cargo test --locked --test github_labels -- --exact the_script_follows_an_open_session_ahead_of_an_archived_one for TC-cli-open-session-sets-the-stage-label: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6bc60913b028, observed 2026-11-21T18:14:10Z. The raw log stays in the machine-local store.
