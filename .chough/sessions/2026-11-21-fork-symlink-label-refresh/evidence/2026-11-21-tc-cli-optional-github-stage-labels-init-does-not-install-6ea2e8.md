---
kind: evidence
id: 2026-11-21-tc-cli-optional-github-stage-labels-init-does-not-install-6ea2e8
aliases: []
claim: "Ran cargo test --locked --test github_labels -- --exact chough_init_does_not_install_the_label_workflow for TC-cli-optional-github-stage-labels-init-does-not-install: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-optional-github-stage-labels-init-does-not-install]
case_digest: [8760f9980d16046a3b64e7f9b6ec481f6aa572808c6703224596e00a9efce75d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - github_labels
  - "--"
  - "--exact"
  - chough_init_does_not_install_the_label_workflow
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 178b66a1607e3d830bbae4de41ce33996f057ef7
content: fa88d8509c33bb33f34fb1a4445868ab8afad994e194116155fdd1d326830d07
environment: local-macos-arm64
fingerprint: sha256:fc491def384ed2052df2d336901b016c46e679e9c9634c617c6eaa959019ee36
criteria: [LIFE-31]
requirements: [REQ-cli-070]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-21T17:35:22Z
actor:
  - terminal: t10j
---
**Ran cargo test --locked --test github_labels -- --exact chough_init_does_not_install_the_label_workflow for TC-cli-optional-github-stage-labels-init-does-not-install: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 178b66a1607e, observed 2026-11-21T17:35:22Z. The raw log stays in the machine-local store.
