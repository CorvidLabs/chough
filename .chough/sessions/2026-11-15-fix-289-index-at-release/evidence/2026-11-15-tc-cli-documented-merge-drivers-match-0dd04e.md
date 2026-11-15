---
kind: evidence
id: 2026-11-15-tc-cli-documented-merge-drivers-match-0dd04e
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
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:cf5c65f9426a04cba95a78fd6870ea4602845c43aeeec69741415dd402d8e132
criteria: [FLOW-18, DRIVE-12]
requirements: [REQ-cli-047]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:16Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:16Z. The raw log stays in the machine-local store.
