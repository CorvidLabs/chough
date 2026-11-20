---
kind: evidence
id: 2026-11-20-tc-cli-every-specsync-pin-names-6-4-0-72629b
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
revision: 600a34b4a7212970c8fa4d39958da116e4f2b9b8
content: f288da34b6490daffc348266c23831006988e58d530541024eaaa57cc05f99ec
environment: local-macos-arm64
fingerprint: sha256:acf4809fb8f11da112d6dd4635cd975d25973de9bb3895fc4629c649fcc07d46
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T23:41:43Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test package -- --exact every_specsync_pin_names_6_4_0 for TC-cli-every-specsync-pin-names-6-4-0: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 600a34b4a721, observed 2026-11-20T23:41:43Z. The raw log stays in the machine-local store.
