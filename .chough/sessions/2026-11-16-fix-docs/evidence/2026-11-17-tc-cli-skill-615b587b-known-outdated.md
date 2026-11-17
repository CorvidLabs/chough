---
kind: evidence
id: 2026-11-17-tc-cli-skill-615b587b-known-outdated
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact skill_files_from_main_615b587b_are_known_outdated_and_edited_copies_stay for TC-cli-skill-615b587b-known-outdated: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-skill-615b587b-known-outdated]
case_digest: [18cc8d5a8c9336ab326b33d150964c9f63024c8434db1503f8353dbd7dc50e34]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - skill_files_from_main_615b587b_are_known_outdated_and_edited_copies_stay
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:123cf61232a71ac2b8f0f91fa3c903b649b8a704ac5fa5fa532a40fc9363e3cd
criteria: [FLOW-18, FLOW-13, LIFE-9]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:27Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test docs -- --exact skill_files_from_main_615b587b_are_known_outdated_and_edited_copies_stay for TC-cli-skill-615b587b-known-outdated: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:27Z. The raw log stays in the machine-local store.
