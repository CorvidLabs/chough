---
kind: evidence
id: 2026-11-13-tc-cli-integrations-readme-3d43b8
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
revision: a442a2acc4a23948c031c36675861ba57f04c980
content: 2fb922c6ab66ac421f2cbd88feaa0ec2fd87004f296b827945dfb087f7231437
environment: local-macos-arm64
fingerprint: sha256:bd0969136be94666e1e375e2d891e5ee068864d415d197b517dfdf7e6ec73d6a
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:54:43Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a442a2acc4a2, observed 2026-11-13T04:54:43Z. The raw log stays in the machine-local store.
