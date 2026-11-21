---
kind: evidence
id: 2026-11-21-tc-cli-keep-applied-stage-label
aliases: []
claim: "Ran cargo test --locked --test github_labels -- --exact the_script_keeps_a_label_it_already_applied for TC-cli-keep-applied-stage-label: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-keep-applied-stage-label]
case_digest: [556e8ded5bf4de337fc22e413147f8ba2fb73f0dde30cddb8a40bafa0efea738]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - github_labels
  - "--"
  - "--exact"
  - the_script_keeps_a_label_it_already_applied
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 727d3bfdeef47b5a5df912d542565f8ba321a377
content: eca3cf6d71aec3d7e6025128737d6b8fee50b0f5e6c80ae342cd16ff6f847418
environment: local-macos-arm64
fingerprint: sha256:bd1168c0dd4c5c1815496a1c1bc7566e0091366982edfd93c7d8b7e5f179e61e
criteria: [LIFE-31]
requirements: [REQ-cli-070]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-21T17:55:36Z
actor:
  - terminal: t10j
---
**Ran cargo test --locked --test github_labels -- --exact the_script_keeps_a_label_it_already_applied for TC-cli-keep-applied-stage-label: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 727d3bfdeef4, observed 2026-11-21T17:55:36Z. The raw log stays in the machine-local store.
