---
kind: evidence
id: 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-ed78e8
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
duration_seconds: 0
revision: 6b7cd4766a52829847a23d03d0d93262d70feeee
content: 49a2ab3c28dbd1ec2012ee1e52591ba22d6e01e8918fc4976e1aaf668257c7a5
environment: local-macos-arm64
fingerprint: sha256:095cd702b5ccee74915bdbc661176ccc9cae52529e8854f61ae225653a23b6d5
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:12:25Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6b7cd4766a52, observed 2026-11-13T03:12:25Z. The raw log stays in the machine-local store.
