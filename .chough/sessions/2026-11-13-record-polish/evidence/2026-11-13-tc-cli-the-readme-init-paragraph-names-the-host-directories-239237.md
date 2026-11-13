---
kind: evidence
id: 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-239237
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly]
case_digest: [f5023a12e5493bcca87a2014014e9a2988bd9a4186712dae93134e87e305b229]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: a1f4735a820dacdb102a4ef4d9c1bda53e31f7bb
content: 65b5bda886fd3fbadb3834aeb5a77124da7d7b9a0d3f00f12ba7d6040b8ce43b
environment: local-macos-arm64
fingerprint: sha256:4e141c53866da2dfea243e1a0f17a17c09b4a5fe7c8615284543dc39c027540a
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:25:50Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a1f4735a820d, observed 2026-11-13T04:25:50Z. The raw log stays in the machine-local store.
