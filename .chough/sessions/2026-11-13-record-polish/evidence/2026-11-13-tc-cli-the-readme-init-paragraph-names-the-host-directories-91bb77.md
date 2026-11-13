---
kind: evidence
id: 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-91bb77
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
duration_seconds: 2
revision: 3453e6e8a2edd34624c6c89b620ed8b262aa3a2c
content: 13e8fbe68ad3139b1c422a706d9abd7d321e4259afebf7c54f9ab96bea045d8e
environment: local-macos-arm64
fingerprint: sha256:d272592c28a5f09895eb0e5a70919c38c199eaa2b49639bf8afd19559e740e89
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:07:50Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3453e6e8a2ed, observed 2026-11-13T06:07:50Z. The raw log stays in the machine-local store.
