---
kind: evidence
id: 2026-11-20-tc-cli-ready-leaves-out-another-change-s-work
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
revision: e8d2ecc43843abdac87b9c439fcbd58f991d112b
content: 8ae2b83a6c451ac9ac8c50f3ac993bc64bf2201b907353f186b31caddaab6902
environment: local-macos-arm64
fingerprint: sha256:27bc28297db5cff584494704d99387914b3b68b6a340dc1a21e1b14f409c2359
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:12:52Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact another_active_change_s_work_is_not_this_change_s for TC-cli-ready-leaves-out-another-change-s-work: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e8d2ecc43843, observed 2026-11-20T19:12:52Z. The raw log stays in the machine-local store.
