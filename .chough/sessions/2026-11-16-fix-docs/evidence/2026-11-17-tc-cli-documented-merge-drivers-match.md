---
kind: evidence
id: 2026-11-17-tc-cli-documented-merge-drivers-match
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-documented-merge-drivers-match]
case_digest: [be40dc133895dbd3617da6f180952bf1aa526c18eb9e7656afda673040129881]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - cli_reference_declares_the_merge_drivers_this_repository_declares
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:63f9b1c19983ae94383201dee113b42d32d229d24c37829cf5aa3c06598091a5
criteria: [FLOW-18, DRIVE-12]
requirements: [REQ-cli-047]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:25Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:25Z. The raw log stays in the machine-local store.
