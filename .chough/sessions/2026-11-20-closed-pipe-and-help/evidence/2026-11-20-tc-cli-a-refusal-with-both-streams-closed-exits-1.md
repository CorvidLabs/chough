---
kind: evidence
id: 2026-11-20-tc-cli-a-refusal-with-both-streams-closed-exits-1
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
revision: be53e668fdf94ba61ff800b3702fc70d89517b82
content: 34322c35469b860b51450cad40a3b0e14c5024c1f945e7dbebfcf2ce4563c892
environment: local-macos-arm64
fingerprint: sha256:5af42604e6754d92e48e9b9563e59e49c90ffa64ae047fa531fe88a844b5226c
criteria: [LIFE-4]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:44:29Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_refusal_with_both_streams_closed_exits_as_it_would for TC-cli-a-refusal-with-both-streams-closed-exits-1: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision be53e668fdf9, observed 2026-11-20T16:44:29Z. The raw log stays in the machine-local store.
