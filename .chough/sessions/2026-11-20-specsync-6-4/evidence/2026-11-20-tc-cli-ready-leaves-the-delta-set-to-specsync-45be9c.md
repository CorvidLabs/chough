---
kind: evidence
id: 2026-11-20-tc-cli-ready-leaves-the-delta-set-to-specsync-45be9c
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_missing_delta_is_named_by_specsync_alone for TC-cli-ready-leaves-the-delta-set-to-specsync: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ready-leaves-the-delta-set-to-specsync]
case_digest: [7684916e4a335a2499cd265362a66fd4439bfb9a0728774a565cb983af6f78f4]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_missing_delta_is_named_by_specsync_alone
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 600a34b4a7212970c8fa4d39958da116e4f2b9b8
content: f288da34b6490daffc348266c23831006988e58d530541024eaaa57cc05f99ec
environment: local-macos-arm64
fingerprint: sha256:17b254b7352e083347a3580963c26f936fc493a8082e80a8b79c4003e5579d01
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T23:41:30Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_missing_delta_is_named_by_specsync_alone for TC-cli-ready-leaves-the-delta-set-to-specsync: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 600a34b4a721, observed 2026-11-20T23:41:30Z. The raw log stays in the machine-local store.
