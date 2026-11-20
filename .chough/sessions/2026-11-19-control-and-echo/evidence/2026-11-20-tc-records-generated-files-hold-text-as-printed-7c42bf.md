---
kind: evidence
id: 2026-11-20-tc-records-generated-files-hold-text-as-printed-7c42bf
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-generated-files-hold-text-as-printed]
case_digest: [cdf17d5ad26155010759ca01fa49e266001d10da788d705e387c20ae3d353b72]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - generated_files_hold_stored_control_characters_as_printed
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
environment: local-macos-arm64
fingerprint: sha256:d1788d72145b5462799c4f1e250c8b05022bd316344d41c2e03deb27381cbaa9
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:12:17Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1406385069c2, observed 2026-11-20T06:12:17Z. The raw log stays in the machine-local store.
