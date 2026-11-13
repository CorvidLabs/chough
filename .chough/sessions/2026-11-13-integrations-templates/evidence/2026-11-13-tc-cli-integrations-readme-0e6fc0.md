---
kind: evidence
id: 2026-11-13-tc-cli-integrations-readme-0e6fc0
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-readme]
case_digest: [8789bd2fb23f68bf4f254a914934273ebf75ccf35a2b5a3b0bb3cd0c336c0440]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - the_readme_names_every_template_and_says_chough_has_no_runtime_dependency
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: c0e3c340ae4fce048f37442831d242d42eeeef28
content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
environment: local-macos-arm64
fingerprint: sha256:58f514756322148ba5a4c97b7a5fc3354eecc713f36c4153616118a64f69fc47
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:19:41Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c0e3c340ae4f, observed 2026-11-13T06:19:41Z. The raw log stays in the machine-local store.
