---
kind: evidence
id: 2026-11-13-tc-cli-integrations-readme-6e82a9
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
revision: 12f2cafe777a48ef6e6cb92c9435997ab5b4e0d2
content: dd44a2f14e6358d057ef934649d20ff96745157b0280619cc43ef2f0e973b2c4
environment: local-macos-arm64
fingerprint: sha256:b909d193093c51fb3856f7acebf5a3fbdc0c363b0a06b8c207515c8349460f6f
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:14:15Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 12f2cafe777a, observed 2026-11-13T04:14:15Z. The raw log stays in the machine-local store.
