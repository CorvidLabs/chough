---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-743-of-743-across-49-targets-cargo
aliases: []
claim: "The full test suite passed 743 of 743 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at 5219fe8d, #253's head with main (#242) merged and the narrowed REQ-cli-065 materialized as cli version 90; the six integrations cases passed at the same content"
result: passed
evidence_kind: product-test
method: cargo test --locked --no-fail-fast under the heavy-tests lock
revision: 5219fe8daacaa2baeb870782955b1f06090c072f
content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
environment: local-macos-arm64
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: []
limitations: []
observed: 2026-11-13T14:55:58Z
actor:
  - agent: claude-code
    terminal: t38r
---
**The full test suite passed 743 of 743 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at 5219fe8d, #253's head with main (#242) merged and the narrowed REQ-cli-065 materialized as cli version 90; the six integrations cases passed at the same content.**

Recorded as passed in local-macos-arm64 at revision 5219fe8daaca, observed 2026-11-13T14:55:58Z.
