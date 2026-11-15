---
kind: evidence
id: 2026-11-15-tc-cli-documented-merge-drivers-match
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
revision: 7321283c6a3194154feab7d635fd2aea08b23efd
content: 88fda8b77b56860cfdb57daff6b1b2b61c6c6e8a3c3c330671846a2a480d6967
environment: local-macos-arm64
fingerprint: sha256:cf5c65f9426a04cba95a78fd6870ea4602845c43aeeec69741415dd402d8e132
criteria: [FLOW-18, DRIVE-12]
requirements: [REQ-cli-047]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:18:11Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7321283c6a31, observed 2026-11-15T20:18:11Z. The raw log stays in the machine-local store.
