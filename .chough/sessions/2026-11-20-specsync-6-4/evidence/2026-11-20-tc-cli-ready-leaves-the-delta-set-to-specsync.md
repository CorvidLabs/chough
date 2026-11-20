---
kind: evidence
id: 2026-11-20-tc-cli-ready-leaves-the-delta-set-to-specsync
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
revision: e8d2ecc43843abdac87b9c439fcbd58f991d112b
content: 8ae2b83a6c451ac9ac8c50f3ac993bc64bf2201b907353f186b31caddaab6902
environment: local-macos-arm64
fingerprint: sha256:9e0e5ada19660db2bb195c08375560562b5b0ef7ff368b29eb8c95c15a9e2b78
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:12:45Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_missing_delta_is_named_by_specsync_alone for TC-cli-ready-leaves-the-delta-set-to-specsync: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e8d2ecc43843, observed 2026-11-20T19:12:45Z. The raw log stays in the machine-local store.
