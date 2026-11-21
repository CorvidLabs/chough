---
kind: evidence
id: 2026-11-21-tc-cli-optional-github-stage-labels
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
duration_seconds: 1
revision: 9f265d9fbb3a914863f1c70fbad6a82ff86a516b
content: 58eff05268add768c550480edc21525594184aa7e3d8c0a4307da857d3224498
environment: local-macos-arm64
fingerprint: sha256:fc6f4960bfe0d73b6a5133b306a9b026eaaa80a5846d7e204ecb704bba96710e
criteria: [LIFE-31]
requirements: [REQ-cli-070]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-21T17:02:31Z
actor:
  - terminal: t10j
---
**Ran cargo test --locked --test github_labels -- --exact the_script_labels_one_main_stage_and_refuses_anything_else for TC-cli-optional-github-stage-labels: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9f265d9fbb3a, observed 2026-11-21T17:02:31Z. The raw log stays in the machine-local store.
