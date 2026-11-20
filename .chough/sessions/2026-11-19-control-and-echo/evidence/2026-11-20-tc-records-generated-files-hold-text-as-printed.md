---
kind: evidence
id: 2026-11-20-tc-records-generated-files-hold-text-as-printed
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
revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
environment: local-macos-arm64
fingerprint: sha256:5be39eefaa39dc1fdec32d703738fc05e1601c542e0c1ceaa6b1826a38d706c1
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:21:27Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d8aa8a63acd5, observed 2026-11-20T03:21:27Z. The raw log stays in the machine-local store.
