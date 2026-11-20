---
kind: evidence
id: 2026-11-20-tc-cli-every-specsync-pin-names-6-4-0-67db23
aliases: []
claim: "Ran cargo test --locked --test package -- --exact every_specsync_pin_names_6_4_0 for TC-cli-every-specsync-pin-names-6-4-0: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-every-specsync-pin-names-6-4-0]
case_digest: [a1c37dc2db69da0eb11ee3413f15afa387b86c609216f3d709dfb2a0e65443d4]
command: [cargo, test, --locked, --test, package, --, --exact, every_specsync_pin_names_6_4_0]
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e5571fd735bf8c4c575ef11a69a63a8e14915bad
content: b4f463f5579645db964f839158eda5550438456bd20e553a4249913a565ca384
environment: local-macos-arm64
fingerprint: sha256:0e68d9f3220616d6dbcb233d3b445c176e8536c93b9a4e1e9723f00b4f3ec80e
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:40:35Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test package -- --exact every_specsync_pin_names_6_4_0 for TC-cli-every-specsync-pin-names-6-4-0: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e5571fd735bf, observed 2026-11-20T19:40:35Z. The raw log stays in the machine-local store.
