---
kind: evidence
id: 2026-11-17-tc-cli-skill-names-advance-to
aliases: []
claim: "Ran cargo test --locked --test package -- --exact the_stages_reference_from_main_01f1d023_is_known_outdated_and_names_advance_to for TC-cli-skill-names-advance-to: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-skill-names-advance-to]
case_digest: [01250bb6acecfac85bd3d6dac1c64806ef10b1c7b8ce30f38ecf9f5aa0ff2287]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - package
  - "--"
  - "--exact"
  - the_stages_reference_from_main_01f1d023_is_known_outdated_and_names_advance_to
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:37370ad243d886d9e74b4dc9778ac3c0028562c492a12361e2204b6c0aa588a3
criteria: [DRIVE-9]
requirements: [REQ-cli-052]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:39Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test package -- --exact the_stages_reference_from_main_01f1d023_is_known_outdated_and_names_advance_to for TC-cli-skill-names-advance-to: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:39Z. The raw log stays in the machine-local store.
