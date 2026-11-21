---
kind: evidence
id: 2026-11-21-tc-cli-optional-github-stage-labels-099eb9
aliases: []
claim: "Ran cargo test --locked --test github_labels -- --exact the_script_labels_one_main_stage_and_refuses_anything_else for TC-cli-optional-github-stage-labels: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-optional-github-stage-labels]
case_digest: [48b35d451c81afec5ef2bdf8cf17f5737f53137f36ae3a5ec0c66ac5a290ba83]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - github_labels
  - "--"
  - "--exact"
  - the_script_labels_one_main_stage_and_refuses_anything_else
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 178b66a1607e3d830bbae4de41ce33996f057ef7
content: fa88d8509c33bb33f34fb1a4445868ab8afad994e194116155fdd1d326830d07
environment: local-macos-arm64
fingerprint: sha256:5e8a7d6e528629510ef65cbbb9938457c26d6da5e4e660c0af2735186de12039
criteria: [LIFE-31]
requirements: [REQ-cli-070]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-21T17:35:20Z
actor:
  - terminal: t10j
---
**Ran cargo test --locked --test github_labels -- --exact the_script_labels_one_main_stage_and_refuses_anything_else for TC-cli-optional-github-stage-labels: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 178b66a1607e, observed 2026-11-21T17:35:20Z. The raw log stays in the machine-local store.
