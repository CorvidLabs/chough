---
kind: evidence
id: 2026-11-20-tc-cli-ready-makes-no-hash-file-d08f07
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
revision: 6571ed05beecf7230bf1ca054a511498a03a3569
content: 3b0af0eee772b0e16f075859206fe29bfddd0832bdb3bdb8464467cea6477800
environment: local-macos-arm64
fingerprint: sha256:062cacdf75b67a7742343017d1c2b9552ec186cd60e1387b7f2ff2ec20502982
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T22:23:11Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact ready_makes_no_hash_file_for_specsync_to_find for TC-cli-ready-makes-no-hash-file: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6571ed05beec, observed 2026-11-20T22:23:11Z. The raw log stays in the machine-local store.
