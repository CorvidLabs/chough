---
kind: evidence
id: 2026-11-13-tc-cli-integrations-readme-c629fa
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
revision: 5219fe8daacaa2baeb870782955b1f06090c072f
content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
environment: local-macos-arm64
fingerprint: sha256:8bd06d0ec36e5e50d408e3850f8b9fb73f97865ae95a6a3b8d15c7b6be4a147d
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T14:50:07Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5219fe8daaca, observed 2026-11-13T14:50:07Z. The raw log stays in the machine-local store.
