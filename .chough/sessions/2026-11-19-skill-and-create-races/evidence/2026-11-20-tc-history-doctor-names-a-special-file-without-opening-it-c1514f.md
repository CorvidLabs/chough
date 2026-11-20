---
kind: evidence
id: 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it-c1514f
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
revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
environment: local-macos-arm64
fingerprint: sha256:1675df3da581591f5def468ace43ab298102b9a4b4bf7cd2ad542bcd135f05e4
criteria: [DRIVE-17]
requirements: [REQ-history-016]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:36:03Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it for TC-history-doctor-names-a-special-file-without-opening-it: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cece9573c0c, observed 2026-11-20T14:36:03Z. The raw log stays in the machine-local store.
