---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-stale-delta-532d38
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-stale-delta]
case_digest: [3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_stale_delta_is_named_until_the_body_is_rewritten
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 6ce76b8b0ab24540e90fff3fa73b68a6113f5ec5
content: cf1c9a338c3fd0cb0280dc103993aa7445a594afc9d339094eab7f408bce818c
environment: local-macos-arm64
fingerprint: sha256:4197e0a27f1a928e64170113b3677a1542c5c696aa90653c0ff6d972a8fa0163
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:53:59Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6ce76b8b0ab2, observed 2026-11-19T04:53:59Z. The raw log stays in the machine-local store.
