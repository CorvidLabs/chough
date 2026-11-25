---
kind: evidence
id: 2026-11-23-tc-cli-grouped-gates-timeline-bound
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_grouped_reply_is_atomic_at_the_real_timeline_cap for TC-cli-grouped-gates-timeline-bound: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-grouped-gates-timeline-bound]
case_digest: [d2885c8e2c0476177dbf6e848f6c578054922fb96d659946bb655eb34392e776]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_grouped_reply_is_atomic_at_the_real_timeline_cap
exit_code: 0
tests_run: 1
duration_seconds: 5
revision: b7fbb1c9d8e3c3d2c39513490920c5a37e5c093f
content: 564cbdf60b148772ee9546ddee6008c971ca697da6924c70263090540a883bdf
environment: local-macos-arm64
fingerprint: sha256:93f0aa9b17f6a3a04dbddf484649f583668ad8eba821121c1596ddbe7547fad5
criteria: [FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-23T15:35:10Z
actor:
  - terminal: t14v
---
**Ran cargo test --locked --test gates -- --exact a_grouped_reply_is_atomic_at_the_real_timeline_cap for TC-cli-grouped-gates-timeline-bound: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b7fbb1c9d8e3, observed 2026-11-23T15:35:10Z. The raw log stays in the machine-local store.
