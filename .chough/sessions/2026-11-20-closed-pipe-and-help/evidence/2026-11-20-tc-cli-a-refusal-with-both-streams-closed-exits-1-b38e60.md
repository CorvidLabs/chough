---
kind: evidence
id: 2026-11-20-tc-cli-a-refusal-with-both-streams-closed-exits-1-b38e60
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact a_refusal_with_both_streams_closed_exits_as_it_would for TC-cli-a-refusal-with-both-streams-closed-exits-1: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-refusal-with-both-streams-closed-exits-1]
case_digest: [8fd125d622dab2989322a1eae1898650455496973f447e9abb4813ba3ad9d8ad]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - a_refusal_with_both_streams_closed_exits_as_it_would
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e60f19787d96c9a0ebcf9399d93a04939d4696fb
content: 1640f508756b07e5571731954b75bcad62ee1ad2bddb05f87a5dd462354e2c55
environment: local-macos-arm64
fingerprint: sha256:e043d0968edeada29fe5b3bb174ac10c61ca1ec8685ed29dfad756e4d046a574
criteria: [LIFE-4]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:07:19Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_refusal_with_both_streams_closed_exits_as_it_would for TC-cli-a-refusal-with-both-streams-closed-exits-1: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e60f19787d96, observed 2026-11-20T17:07:19Z. The raw log stays in the machine-local store.
