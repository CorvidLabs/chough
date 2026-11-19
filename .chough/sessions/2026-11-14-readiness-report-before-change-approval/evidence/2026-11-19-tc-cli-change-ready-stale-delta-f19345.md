---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-stale-delta-f19345
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
revision: 5b9c1b4ad6f29a5f973182a39ee97c96f3a4285a
content: 236b6f9b5fdb32badd2454e8416b7615cd2602a4f3662453783ffb6f03796737
environment: local-macos-arm64
fingerprint: sha256:79b09dad45278d24d326a185cfe6f289f1eff0d0cd16bb691c89d454f8c101d0
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:14:56Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5b9c1b4ad6f2, observed 2026-11-19T15:14:56Z. The raw log stays in the machine-local store.
