---
kind: evidence
id: 2026-11-20-tc-cli-ready-makes-no-hash-file-3b08a5
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
duration_seconds: 1
revision: 600a34b4a7212970c8fa4d39958da116e4f2b9b8
content: f288da34b6490daffc348266c23831006988e58d530541024eaaa57cc05f99ec
environment: local-macos-arm64
fingerprint: sha256:39e6a719d57d7e6243f80e94f2fb57eb6c04d46062b9b33d9969dfe3b072913f
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T23:41:41Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact ready_makes_no_hash_file_for_specsync_to_find for TC-cli-ready-makes-no-hash-file: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 600a34b4a721, observed 2026-11-20T23:41:41Z. The raw log stays in the machine-local store.
