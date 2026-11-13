---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-740-of-740-across-49-targets-cargo
aliases: []
claim: "The full test suite passed 740 of 740 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at a442a2ac, #219's head with main (#248) merged and REQ-cli-065 materialized; clippy on the integrations test with -D warnings and rustfmt --check were clean"
result: passed
evidence_kind: product-test
method: cargo clippy --locked --test integrations -- -D warnings; cargo test --locked --no-fail-fast; under the heavy-tests lock
revision: a442a2acc4a23948c031c36675861ba57f04c980
content: e4635ebf3f79f4f68a66e0b586fa29e2568f544b8874addb64602529ea1095c9
environment: local-macos-arm64
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: []
limitations: []
observed: 2026-11-13T05:03:10Z
actor:
  - agent: claude-code
    terminal: t38r
---
**The full test suite passed 740 of 740 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at a442a2ac, #219's head with main (#248) merged and REQ-cli-065 materialized; clippy on the integrations test with -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision a442a2acc4a2, observed 2026-11-13T05:03:10Z.
