---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-731-of-731-across-48-targets-cargo
aliases: []
claim: The full test suite passed 731 of 731 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 8e0feede with REQ-cli-061's new text materialized; clippy --all-targets -D warnings and rustfmt --check were clean
result: passed
evidence_kind: product-test
method: automated
revision: 8e0feedecb1cbe101c723067a5c300d2cbad07fc
content: a4cfc78f0189db0c9bae41beff3e0201fb02123516fe566e98c9dd7f65c48295
environment: local-macos-arm64
criteria: [FLOW-19]
requirements: []
contracts: []
limitations:
  - Run on the working tree before the materialized text was committed
  - The raw log is not committed
observed: 2026-11-13T02:34:20Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 731 of 731 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 8e0feede with REQ-cli-061's new text materialized; clippy --all-targets -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision 8e0feedecb1c, observed 2026-11-13T02:34:20Z.
