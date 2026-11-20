---
kind: evidence
id: 2026-11-20-tc-cli-ready-leaves-out-another-change-s-work-ffa494
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact another_active_change_s_work_is_not_this_change_s for TC-cli-ready-leaves-out-another-change-s-work: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ready-leaves-out-another-change-s-work]
case_digest: [6decafa46db6e465cff095580cfd5e81e46f678925516ceae72bee6db99c2177]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - another_active_change_s_work_is_not_this_change_s
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e5571fd735bf8c4c575ef11a69a63a8e14915bad
content: b4f463f5579645db964f839158eda5550438456bd20e553a4249913a565ca384
environment: local-macos-arm64
fingerprint: sha256:1ba29d731634e6830ad2038e2b1b24d6dde00154d0292192eb33de615243de23
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:40:31Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact another_active_change_s_work_is_not_this_change_s for TC-cli-ready-leaves-out-another-change-s-work: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e5571fd735bf, observed 2026-11-20T19:40:31Z. The raw log stays in the machine-local store.
