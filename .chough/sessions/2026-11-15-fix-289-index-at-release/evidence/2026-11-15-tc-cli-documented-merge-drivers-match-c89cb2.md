---
kind: evidence
id: 2026-11-15-tc-cli-documented-merge-drivers-match-c89cb2
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
revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
environment: local-macos-arm64
fingerprint: sha256:8fdca87a7de51458a4a15b3c70aac5773bd7ed5d39f09e0b3f0c8a31807e02cf
criteria: [FLOW-18, DRIVE-12]
requirements: [REQ-cli-047]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T21:09:08Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8b5bd53cab53, observed 2026-11-15T21:09:08Z. The raw log stays in the machine-local store.
