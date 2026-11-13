---
kind: evidence
id: 2026-11-13-fledge-lanes-run-verify-fledge-1-8-0-the-ci-pin-in-one-pass-at
aliases: []
claim: "fledge lanes run verify (Fledge 1.8.0, the CI pin) in one pass at bf753857 with a clean tree under the heavy-tests lock: toolchain, fmt, lint, test (735 passed in 48 targets, 0 failed), intent, spec, audit, cases, knowledge, leaks, exercise and build all passed."
result: passed
evidence_kind: product-test
revision: bf7538571b47d0fcf1ca2d16c90305bd9ec68e8c
content: 37601ca808fa35558fbf1b9a6c5612b97deaf5d05c594db7a389cd36732b8592
environment: local-macos-arm64
criteria: [LIFE-21, FLOW-21]
requirements: [REQ-cli-066]
contracts: []
limitations: [macOS arm64; hosted CI runs Ubuntu (dash) and macOS after the change is archived]
observed: 2026-11-13T07:39:56Z
actor:
  - agent: claude-code
    model: claude-opus-5-5
    terminal: t38q
---
**fledge lanes run verify (Fledge 1.8.0, the CI pin) in one pass at bf753857 with a clean tree under the heavy-tests lock: toolchain, fmt, lint, test (735 passed in 48 targets, 0 failed), intent, spec, audit, cases, knowledge, leaks, exercise and build all passed.**

Recorded as passed in local-macos-arm64 at revision bf7538571b47, observed 2026-11-13T07:39:56Z.
