---
kind: evidence
id: 2026-11-20-tc-cli-ready-leaves-the-delta-set-to-specsync-392209
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
revision: e5571fd735bf8c4c575ef11a69a63a8e14915bad
content: b4f463f5579645db964f839158eda5550438456bd20e553a4249913a565ca384
environment: local-macos-arm64
fingerprint: sha256:5b4ea9c157067c30a0b5a5df6a1bc6d379da188f01f84a4c71732d901c24dd8c
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:40:23Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_missing_delta_is_named_by_specsync_alone for TC-cli-ready-leaves-the-delta-set-to-specsync: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e5571fd735bf, observed 2026-11-20T19:40:23Z. The raw log stays in the machine-local store.
