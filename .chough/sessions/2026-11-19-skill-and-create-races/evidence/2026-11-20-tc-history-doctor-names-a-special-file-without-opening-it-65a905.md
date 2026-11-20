---
kind: evidence
id: 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it-65a905
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
revision: f38abb556cee50e587a4c2446a91c7f2fa140997
content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
environment: local-macos-arm64
fingerprint: sha256:1675df3da581591f5def468ace43ab298102b9a4b4bf7cd2ad542bcd135f05e4
criteria: [DRIVE-17]
requirements: [REQ-history-016]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T15:00:12Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it for TC-history-doctor-names-a-special-file-without-opening-it: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f38abb556cee, observed 2026-11-20T15:00:12Z. The raw log stays in the machine-local store.
