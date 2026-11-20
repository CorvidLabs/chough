---
kind: evidence
id: 2026-11-20-tc-cli-ready-leaves-out-another-change-s-work-ce7f95
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
revision: 6571ed05beecf7230bf1ca054a511498a03a3569
content: 3b0af0eee772b0e16f075859206fe29bfddd0832bdb3bdb8464467cea6477800
environment: local-macos-arm64
fingerprint: sha256:6f8a0765b45b73745f1f27f55216fdbe3294b89bfb552de6b7e91a1351f4f616
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T22:23:09Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact another_active_change_s_work_is_not_this_change_s for TC-cli-ready-leaves-out-another-change-s-work: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6571ed05beec, observed 2026-11-20T22:23:09Z. The raw log stays in the machine-local store.
