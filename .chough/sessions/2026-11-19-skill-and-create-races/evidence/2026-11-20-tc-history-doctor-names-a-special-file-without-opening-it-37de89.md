---
kind: evidence
id: 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it-37de89
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it for TC-history-doctor-names-a-special-file-without-opening-it: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-doctor-names-a-special-file-without-opening-it]
case_digest: [bae764610596635f70c93a50d78adfde336d28e78d96e756b798d93373b58e92]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - doctor_names_a_special_file_without_opening_it
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 29fb39131c6435612182f52929b94e69ab0dcba3
content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
environment: local-macos-arm64
fingerprint: sha256:617698c61a8f67d8188d259ef00f113854411c404094c1a4eaddfb5eddea9be9
criteria: [DRIVE-17]
requirements: [REQ-history-016]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:19:45Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it for TC-history-doctor-names-a-special-file-without-opening-it: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 29fb39131c64, observed 2026-11-20T14:19:45Z. The raw log stays in the machine-local store.
