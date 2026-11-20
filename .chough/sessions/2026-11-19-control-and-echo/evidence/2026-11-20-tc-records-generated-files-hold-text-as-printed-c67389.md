---
kind: evidence
id: 2026-11-20-tc-records-generated-files-hold-text-as-printed-c67389
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
revision: cdc0d2e2f85a6d09567b8780662656ccec551526
content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
environment: local-macos-arm64
fingerprint: sha256:dd83ecee7bb4149dc2c5f95e89e1bc3b3fafc63c4c57d5eba46cf1dab7c97f2a
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:34:09Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cdc0d2e2f85a, observed 2026-11-20T04:34:09Z. The raw log stays in the machine-local store.
