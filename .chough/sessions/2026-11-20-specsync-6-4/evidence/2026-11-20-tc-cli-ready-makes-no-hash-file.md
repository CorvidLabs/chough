---
kind: evidence
id: 2026-11-20-tc-cli-ready-makes-no-hash-file
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact ready_makes_no_hash_file_for_specsync_to_find for TC-cli-ready-makes-no-hash-file: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ready-makes-no-hash-file]
case_digest: [93c1d2c9d2a83eaf640833abdf9bad72483e23a25e60538479ab610da71c7356]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - ready_makes_no_hash_file_for_specsync_to_find
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e8d2ecc43843abdac87b9c439fcbd58f991d112b
content: 8ae2b83a6c451ac9ac8c50f3ac993bc64bf2201b907353f186b31caddaab6902
environment: local-macos-arm64
fingerprint: sha256:5893dc8f58ab400599f39ffb1822938ed0d660d518ced1cbd0fe65a962dc4ebb
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:12:54Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact ready_makes_no_hash_file_for_specsync_to_find for TC-cli-ready-makes-no-hash-file: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e8d2ecc43843, observed 2026-11-20T19:12:54Z. The raw log stays in the machine-local store.
