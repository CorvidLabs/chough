---
kind: evidence
id: 2026-11-17-tc-cli-inline-diagrams-match-sources
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact inline_diagrams_equal_their_sources_and_svgs_carry_every_label for TC-cli-inline-diagrams-match-sources: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-inline-diagrams-match-sources]
case_digest: [ba2ebb9f9acf2cdbd35c734e3d01bcb2cbfc658dcba5513f5beed0218e10eb54]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - inline_diagrams_equal_their_sources_and_svgs_carry_every_label
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:6c20663bdd8f802d8d9eb896cd7f1eb860dbc4c108983b8f6c95ceff902b08f1
criteria: [FLOW-18]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:24Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test docs -- --exact inline_diagrams_equal_their_sources_and_svgs_carry_every_label for TC-cli-inline-diagrams-match-sources: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:24Z. The raw log stays in the machine-local store.
