---
kind: evidence
id: 2026-11-15-tc-cli-librarian-default-on-knowledge
aliases: []
claim: "Ran cargo test --locked --test optional -- --exact the_librarian_is_on_by_default_in_knowledge_and_can_be_turned_off for TC-cli-librarian-default-on-knowledge: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-librarian-default-on-knowledge]
case_digest: [d31d4fec5a25ef0a00679b5da036f2d993503f705b04b830c18fef004737c00b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - optional
  - "--"
  - "--exact"
  - the_librarian_is_on_by_default_in_knowledge_and_can_be_turned_off
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:ccf1c86c0a149788b17179cb0b49ad3097eb08ff8e581c88e9b2c57196856bf3
criteria: [KNOW-1, KNOW-5]
requirements: [REQ-cli-015]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:05Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test optional -- --exact the_librarian_is_on_by_default_in_knowledge_and_can_be_turned_off for TC-cli-librarian-default-on-knowledge: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:05Z. The raw log stays in the machine-local store.
