---
kind: evidence
id: 2026-11-21-tc-cli-optional-github-stage-labels-init-does-not-install
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
revision: 9f265d9fbb3a914863f1c70fbad6a82ff86a516b
content: 58eff05268add768c550480edc21525594184aa7e3d8c0a4307da857d3224498
environment: local-macos-arm64
fingerprint: sha256:b8e81b6b250eb5331a169dce918587c48c7535d4a072eaa1b33c9b43ff59231b
criteria: [LIFE-31]
requirements: [REQ-cli-070]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-21T17:02:32Z
actor:
  - terminal: t10j
---
**Ran cargo test --locked --test github_labels -- --exact chough_init_does_not_install_the_label_workflow for TC-cli-optional-github-stage-labels-init-does-not-install: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9f265d9fbb3a, observed 2026-11-21T17:02:32Z. The raw log stays in the machine-local store.
